use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::config::UnitsConfig;
use crate::models::HourlyForecast;
use crate::ui::icons::temperature_color_celsius;
use chrono::{Local, NaiveDateTime, Timelike};

const CHART_HEIGHT: usize = 8;

pub fn render_today_chart(
    frame: &mut Frame,
    area: Rect,
    hourly: &[HourlyForecast],
    units: &UnitsConfig,
) {
    let block = Block::default()
        .title(" Today's Forecast ")
        .title_style(
            Style::default()
                .fg(Color::Green)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    let now = Local::now();
    let today = now.date_naive();
    let current_hour = now.hour();

    // Get all of today's hourly data (full 24 hours)
    let today_hours: Vec<&HourlyForecast> = hourly
        .iter()
        .filter(|h| {
            if let Ok(dt) = NaiveDateTime::parse_from_str(&h.time, "%Y-%m-%dT%H:%M") {
                dt.date() == today
            } else {
                false
            }
        })
        .collect();

    if today_hours.is_empty() || inner.width < 20 || inner.height < 6 {
        let msg = Paragraph::new("Not enough data");
        frame.render_widget(msg, inner);
        return;
    }

    // Find temperature range (in Celsius - raw API data)
    let temps: Vec<f64> = today_hours.iter().map(|h| h.temperature).collect();
    let temp_min_c = temps.iter().cloned().fold(f64::INFINITY, f64::min);
    let temp_max_c = temps.iter().cloned().fold(f64::NEG_INFINITY, f64::max);
    let temp_range = (temp_max_c - temp_min_c).max(1.0);

    // Convert min/max to user's preferred unit for display labels
    let temp_min_display = units.temperature.convert(temp_min_c);
    let temp_max_display = units.temperature.convert(temp_max_c);

    // Calculate available width for the chart (leave room for labels)
    let label_width = 6; // "100% " or " 72° "
    let chart_width = (inner.width as usize).saturating_sub(label_width);

    if chart_width == 0 || today_hours.is_empty() {
        return;
    }

    // Calculate how many characters per hour we can use
    let chars_per_hour = (chart_width / today_hours.len()).max(1);
    let total_hours = today_hours.len();

    // Build the chart
    let mut lines: Vec<Line> = Vec::new();

    // Chart rows (from top to bottom: high temp to low temp)
    for row in 0..CHART_HEIGHT {
        let mut spans: Vec<Span> = Vec::new();

        // Left label (in user's preferred unit)
        if row == 0 {
            spans.push(Span::styled(
                format!("{:>5}", temp_max_display as i32),
                Style::default().fg(Color::DarkGray),
            ));
        } else if row == CHART_HEIGHT - 1 {
            spans.push(Span::styled(
                format!("{:>5}", temp_min_display as i32),
                Style::default().fg(Color::DarkGray),
            ));
        } else {
            spans.push(Span::raw("     "));
        }

        // Separator
        spans.push(Span::styled("│", Style::default().fg(Color::DarkGray)));

        // Chart content - draw each hour with proper spacing
        for (i, hour) in today_hours.iter().enumerate() {
            let hour_num =
                if let Ok(dt) = NaiveDateTime::parse_from_str(&hour.time, "%Y-%m-%dT%H:%M") {
                    dt.hour()
                } else {
                    0
                };

            let is_current = hour_num == current_hour;

            // Calculate temperature position (0 = bottom, CHART_HEIGHT-1 = top)
            // Use raw Celsius values for consistent positioning
            let temp_normalized = (hour.temperature - temp_min_c) / temp_range;
            let temp_row = ((CHART_HEIGHT - 1) as f64 * (1.0 - temp_normalized)).round() as usize;

            // Calculate rain position
            let rain_normalized = hour.precipitation_probability as f64 / 100.0;
            let rain_row = ((CHART_HEIGHT - 1) as f64 * (1.0 - rain_normalized)).round() as usize;

            // Determine what to draw at this position
            let (ch, color) = if row == temp_row && row == rain_row {
                // Both temp and rain at same position
                (
                    '◆',
                    if is_current {
                        Color::White
                    } else {
                        Color::Yellow
                    },
                )
            } else if row == temp_row {
                // Temperature point - use raw Celsius value
                let temp_color = temperature_color_celsius(hour.temperature);
                ('●', if is_current { Color::White } else { temp_color })
            } else if row == rain_row {
                // Rain point
                let rain_color = rain_to_color(hour.precipitation_probability);
                ('○', if is_current { Color::White } else { rain_color })
            } else {
                (' ', Color::DarkGray)
            };

            let style = if is_current {
                Style::default().fg(color).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(color)
            };

            // Draw the character centered in its slot
            let padding_before = (chars_per_hour - 1) / 2;
            let padding_after = chars_per_hour - 1 - padding_before;

            if padding_before > 0 {
                spans.push(Span::raw(" ".repeat(padding_before)));
            }
            spans.push(Span::styled(ch.to_string(), style));
            if padding_after > 0 && i < total_hours - 1 {
                spans.push(Span::raw(" ".repeat(padding_after)));
            }
        }

        lines.push(Line::from(spans));
    }

    // Add hour labels at bottom
    let mut hour_spans: Vec<Span> = Vec::new();
    hour_spans.push(Span::raw("     ")); // Label spacer
    hour_spans.push(Span::styled("└", Style::default().fg(Color::DarkGray)));

    for (i, hour) in today_hours.iter().enumerate() {
        let hour_num = if let Ok(dt) = NaiveDateTime::parse_from_str(&hour.time, "%Y-%m-%dT%H:%M") {
            dt.hour()
        } else {
            0
        };

        let is_current = hour_num == current_hour;

        // Show hour label at key hours or current
        let show_label = hour_num % 6 == 0 || is_current;

        let label_char = if show_label {
            if hour_num == 0 {
                "0"
            } else if hour_num == 6 {
                "6"
            } else if hour_num == 12 {
                "N"
            } else if hour_num == 18 {
                "6"
            } else {
                &format!("{}", hour_num % 12)
            }
        } else {
            "─"
        };

        let style = if is_current {
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else if show_label {
            Style::default().fg(Color::Gray)
        } else {
            Style::default().fg(Color::DarkGray)
        };

        // Center label in slot
        let padding_before = (chars_per_hour - 1) / 2;
        let padding_after = chars_per_hour - 1 - padding_before;

        if padding_before > 0 {
            hour_spans.push(Span::styled(
                "─".repeat(padding_before),
                Style::default().fg(Color::DarkGray),
            ));
        }
        hour_spans.push(Span::styled(label_char.to_string(), style));
        if padding_after > 0 && i < total_hours - 1 {
            hour_spans.push(Span::styled(
                "─".repeat(padding_after),
                Style::default().fg(Color::DarkGray),
            ));
        }
    }
    lines.push(Line::from(hour_spans));

    // Add legend
    lines.push(Line::from(vec![
        Span::raw("      "),
        Span::styled("●", Style::default().fg(Color::Yellow)),
        Span::styled(" Temp  ", Style::default().fg(Color::DarkGray)),
        Span::styled("○", Style::default().fg(Color::Cyan)),
        Span::styled(" Rain  ", Style::default().fg(Color::DarkGray)),
        Span::styled("0", Style::default().fg(Color::Gray)),
        Span::styled("=12am ", Style::default().fg(Color::DarkGray)),
        Span::styled("N", Style::default().fg(Color::Gray)),
        Span::styled("=Noon ", Style::default().fg(Color::DarkGray)),
        Span::styled("6", Style::default().fg(Color::Gray)),
        Span::styled("=6am/pm", Style::default().fg(Color::DarkGray)),
    ]));

    let chart = Paragraph::new(lines);
    frame.render_widget(chart, inner);
}

fn rain_to_color(rain: i32) -> Color {
    match rain {
        0..=20 => Color::Green,
        21..=50 => Color::Yellow,
        51..=70 => Color::Rgb(255, 165, 0),
        _ => Color::Cyan,
    }
}

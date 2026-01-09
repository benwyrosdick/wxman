use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::config::UnitsConfig;
use crate::models::HourlyForecast;
use crate::ui::icons::{temperature_color_celsius, WeatherCondition};
use chrono::{DateTime, Datelike, Local, NaiveDateTime, TimeZone, Timelike};

pub fn render_hourly_forecast(
    frame: &mut Frame,
    area: Rect,
    hourly: &[HourlyForecast],
    units: &UnitsConfig,
    scroll_offset: usize,
) {
    let block = Block::default()
        .title(" Hourly Forecast ")
        .title_style(
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Get current hour to filter past hours
    let now = Local::now();

    // Filter to show only future hours (including current hour)
    let future_hours: Vec<&HourlyForecast> = hourly
        .iter()
        .filter(|h| {
            if let Ok(dt) = NaiveDateTime::parse_from_str(&h.time, "%Y-%m-%dT%H:%M") {
                let local_dt = Local.from_local_datetime(&dt).single();
                if let Some(local_dt) = local_dt {
                    return local_dt >= now - chrono::Duration::hours(1);
                }
            }
            false
        })
        .collect();

    // Calculate how many hours we can display (subtract 2 for header and separator)
    let available_height = inner.height as usize;
    let hours_to_show = available_height.saturating_sub(2);

    let mut lines = Vec::new();

    // Add table header
    let header_style = Style::default()
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD);
    lines.push(Line::from(vec![
        Span::styled(format!("{:<10}", "Date"), header_style),
        Span::styled(format!("{:>6}", "Time"), header_style),
        Span::styled(format!("{:>6}", "Temp"), header_style),
        Span::styled(format!("{:>8}", "Feels"), header_style),
        Span::styled(format!("{:>4}", ""), header_style),
        Span::styled(format!("{:>8}", "Wind"), header_style),
        Span::styled(format!("{:>12}", "Precip"), header_style),
        // Span::styled(format!("{:>6}", ""), header_style),
    ]));

    // Add separator line
    lines.push(Line::from(Span::styled(
        "─".repeat(inner.width as usize),
        Style::default().fg(Color::DarkGray),
    )));

    // Track the current date being displayed
    let mut last_date: Option<(i32, u32, u32)> = None;

    // Determine if first visible row should show date
    if let Some(first_hour) = future_hours.get(scroll_offset) {
        if let Ok(dt) = NaiveDateTime::parse_from_str(&first_hour.time, "%Y-%m-%dT%H:%M") {
            let local_dt: DateTime<Local> = Local
                .from_local_datetime(&dt)
                .single()
                .unwrap_or_else(Local::now);
            last_date = Some((local_dt.year(), local_dt.month(), local_dt.day()));
        }
    }

    // Track if we've shown the date for the first visible row
    let mut first_row_date_shown = false;

    for (i, hour) in future_hours
        .iter()
        .skip(scroll_offset)
        .take(hours_to_show)
        .enumerate()
    {
        // Parse time
        let parsed_dt = NaiveDateTime::parse_from_str(&hour.time, "%Y-%m-%dT%H:%M").ok();
        let local_dt: Option<DateTime<Local>> =
            parsed_dt.and_then(|dt| Local.from_local_datetime(&dt).single());

        let (time_str, is_midnight, current_date) = if let Some(ldt) = local_dt {
            let is_midnight = ldt.hour() == 0;
            let date_tuple = (ldt.year(), ldt.month(), ldt.day());
            let time_formatted = ldt.format("%l%p").to_string().trim().to_string();
            (time_formatted, is_midnight, Some(date_tuple))
        } else {
            (hour.time.clone(), false, None)
        };

        // Determine if this is during day (rough estimate: 6am-8pm)
        let is_day = local_dt
            .map(|ldt| (6..=20).contains(&ldt.hour()))
            .unwrap_or(true);

        let condition = WeatherCondition::from_wmo_code(hour.weather_code, is_day);

        // Convert temperature from Celsius to user's preferred unit
        let temp = units.temperature.convert(hour.temperature);
        let feels_like = units.temperature.convert(hour.apparent_temperature);
        // Get color based on raw Celsius value
        let temp_color = temperature_color_celsius(hour.temperature);

        // Convert wind speed from km/h to user's preferred unit
        let wind_speed = units.wind_speed.convert(hour.wind_speed);

        // Color precipitation probability
        let precip_color = match hour.precipitation_probability {
            0..=20 => Color::Green,
            21..=50 => Color::Yellow,
            51..=70 => Color::Rgb(255, 165, 0),
            _ => Color::Red,
        };

        // Convert precipitation amount from mm to user's preferred unit
        let precip_amount = units.precipitation.convert(hour.precipitation);
        let precip_str = format!("{:>3}%", hour.precipitation_probability);

        let precip_amount_str = if hour.precipitation > 0.0 {
            format!("{:>6.2} {}", precip_amount, units.precipitation.symbol())
        } else {
            "".to_string()
        };

        // Determine if we should show date in the date column
        // Show on first row, or at midnight when date changes
        let show_date = if i == 0 && !first_row_date_shown {
            first_row_date_shown = true;
            true
        } else if is_midnight {
            // Show date at midnight if it's a new day
            if let (Some(curr), Some(prev)) = (current_date, last_date) {
                curr != prev
            } else {
                true
            }
        } else {
            false
        };

        // Update last_date
        if let Some(curr) = current_date {
            last_date = Some(curr);
        }

        // Build the date column
        let date_col = if show_date {
            if let Some(ldt) = local_dt {
                ldt.format("%a %m/%d").to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

        let date_style = Style::default()
            .fg(Color::Magenta)
            .add_modifier(Modifier::BOLD);

        // Highlight current hour (first row when not scrolled)
        let time_style = if i == 0 && scroll_offset == 0 {
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };

        lines.push(Line::from(vec![
            Span::styled(format!("{:<10}", date_col), date_style),
            Span::styled(format!("{:>6}", time_str), time_style),
            Span::styled(
                format!("{:>5}°", temp as i32),
                Style::default().fg(temp_color),
            ),
            Span::styled(
                format!("{:>5}°", feels_like as i32),
                Style::default().fg(Color::Gray),
            ),
            Span::styled(
                format!("  {} ", condition.small_icon()),
                Style::default().fg(condition.color()),
            ),
            Span::styled(
                format!("{:>7.0} {}", wind_speed, units.wind_speed.symbol()),
                Style::default().fg(Color::LightGreen),
            ),
            Span::styled(
                format!("{:>6}", precip_str),
                Style::default().fg(precip_color),
            ),
            Span::styled(
                format!("{:>6}", precip_amount_str),
                Style::default().fg(precip_color),
            ),
        ]));
    }

    // Add scroll indicator if there are more items
    let total_future = future_hours.len();
    if scroll_offset + hours_to_show < total_future && lines.len() < available_height {
        lines.push(Line::from(Span::styled(
            "          ↓ more",
            Style::default().fg(Color::DarkGray),
        )));
    }

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, inner);
}

/// Get the maximum scroll offset for hourly forecast
pub fn get_max_hourly_scroll(hourly: &[HourlyForecast], visible_height: usize) -> usize {
    let now = Local::now();

    let future_count = hourly
        .iter()
        .filter(|h| {
            if let Ok(dt) = NaiveDateTime::parse_from_str(&h.time, "%Y-%m-%dT%H:%M") {
                let local_dt = Local.from_local_datetime(&dt).single();
                if let Some(local_dt) = local_dt {
                    return local_dt >= now - chrono::Duration::hours(1);
                }
            }
            false
        })
        .count();

    // Account for header (2 lines)
    future_count.saturating_sub(visible_height.saturating_sub(2))
}

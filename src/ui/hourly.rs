use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::config::UnitsConfig;
use crate::models::HourlyForecast;
use crate::ui::icons::{temperature_color, WeatherCondition};
use chrono::{DateTime, Local, NaiveDateTime, TimeZone};

pub fn render_hourly_forecast(
    frame: &mut Frame,
    area: Rect,
    hourly: &[HourlyForecast],
    units: &UnitsConfig,
    scroll_offset: usize,
) {
    let block = Block::default()
        .title(" Hourly Forecast ")
        .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
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

    // Calculate how many hours we can display
    let available_height = inner.height as usize;
    let hours_to_show = available_height.saturating_sub(1); // Leave room for scroll indicator
    
    let is_fahrenheit = units.temperature == crate::config::TemperatureUnit::Fahrenheit;

    let mut lines = Vec::new();

    for (i, hour) in future_hours
        .iter()
        .skip(scroll_offset)
        .take(hours_to_show)
        .enumerate()
    {
        // Parse time to get hour
        let time_str = if let Ok(dt) = NaiveDateTime::parse_from_str(&hour.time, "%Y-%m-%dT%H:%M") {
            let local_dt: DateTime<Local> = Local.from_local_datetime(&dt).single().unwrap_or_else(|| Local::now());
            local_dt.format("%l%p").to_string().trim().to_string()
        } else {
            hour.time.clone()
        };

        // Determine if this is during day (rough estimate: 6am-8pm)
        let is_day = if let Ok(dt) = NaiveDateTime::parse_from_str(&hour.time, "%Y-%m-%dT%H:%M") {
            let hour_of_day = dt.format("%H").to_string().parse::<u32>().unwrap_or(12);
            (6..=20).contains(&hour_of_day)
        } else {
            true
        };

        let condition = WeatherCondition::from_wmo_code(hour.weather_code, is_day);
        let temp_color = temperature_color(hour.temperature, is_fahrenheit);

        // Color precipitation probability
        let precip_color = match hour.precipitation_probability {
            0..=20 => Color::Green,
            21..=50 => Color::Yellow,
            51..=70 => Color::Rgb(255, 165, 0),
            _ => Color::Red,
        };

        // Highlight current hour
        let time_style = if i == 0 && scroll_offset == 0 {
            Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };

        lines.push(Line::from(vec![
            Span::styled(format!("{:>5} ", time_str), time_style),
            Span::styled(
                format!("{:>3}° ", hour.temperature as i32),
                Style::default().fg(temp_color),
            ),
            Span::styled(
                format!("{} ", condition.small_icon()),
                Style::default().fg(condition.color()),
            ),
            Span::styled(
                format!("{:>3}%", hour.precipitation_probability),
                Style::default().fg(precip_color),
            ),
        ]));
    }

    // Add scroll indicator if there are more items
    let total_future = future_hours.len();
    if scroll_offset + hours_to_show < total_future && lines.len() < available_height {
        lines.push(Line::from(Span::styled(
            "    ↓ more",
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

    future_count.saturating_sub(visible_height.saturating_sub(1))
}

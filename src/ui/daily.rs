use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::config::UnitsConfig;
use crate::models::DailyForecast;
use crate::ui::icons::{temperature_color, uv_info, WeatherCondition};
use chrono::NaiveDate;

pub fn render_daily_forecast(
    frame: &mut Frame,
    area: Rect,
    daily: &[DailyForecast],
    units: &UnitsConfig,
) {
    let block = Block::default()
        .title(" 5-Day Forecast ")
        .title_style(Style::default().fg(Color::Magenta).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Magenta));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Take only first 5 days
    let days: Vec<&DailyForecast> = daily.iter().take(5).collect();
    
    if days.is_empty() {
        return;
    }

    // Create columns for each day
    let constraints: Vec<Constraint> = days
        .iter()
        .map(|_| Constraint::Ratio(1, days.len() as u32))
        .collect();

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(constraints)
        .split(inner);

    let is_fahrenheit = units.temperature == crate::config::TemperatureUnit::Fahrenheit;

    for (i, day) in days.iter().enumerate() {
        render_day_column(frame, chunks[i], day, units, is_fahrenheit, i == 0);
    }
}

fn render_day_column(
    frame: &mut Frame,
    area: Rect,
    day: &DailyForecast,
    units: &UnitsConfig,
    is_fahrenheit: bool,
    is_today: bool,
) {
    let condition = WeatherCondition::from_wmo_code(day.weather_code, true);
    let icon = condition.icon();

    // Parse date
    let date_str = if let Ok(date) = NaiveDate::parse_from_str(&day.date, "%Y-%m-%d") {
        if is_today {
            "Today".to_string()
        } else {
            date.format("%a %m/%d").to_string()
        }
    } else {
        day.date.clone()
    };

    // Temperature colors
    let high_color = temperature_color(day.temp_max, is_fahrenheit);
    let low_color = temperature_color(day.temp_min, is_fahrenheit);

    // Precipitation color
    let precip_color = match day.precipitation_probability {
        0..=20 => Color::Green,
        21..=50 => Color::Yellow,
        51..=70 => Color::Rgb(255, 165, 0),
        _ => Color::Red,
    };

    let (uv_desc, uv_color) = uv_info(day.uv_index_max);

    let mut lines = Vec::new();

    // Day header
    let header_style = if is_today {
        Style::default().fg(Color::White).add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Gray)
    };
    lines.push(Line::from(Span::styled(
        format!("{:^width$}", date_str, width = area.width as usize),
        header_style,
    )));

    lines.push(Line::from(""));

    // Weather icon (centered)
    for line in icon.iter() {
        let padding = (area.width as usize).saturating_sub(line.len()) / 2;
        lines.push(Line::from(Span::styled(
            format!("{:>padding$}{}", "", line, padding = padding),
            Style::default().fg(condition.color()),
        )));
    }

    lines.push(Line::from(""));

    // Low/High temperature
    let temp_line = format!(
        "{}° / {}°",
        day.temp_min as i32,
        day.temp_max as i32
    );
    let padding = (area.width as usize).saturating_sub(temp_line.len()) / 2;
    lines.push(Line::from(vec![
        Span::raw(format!("{:>padding$}", "", padding = padding)),
        Span::styled(
            format!("{}°", day.temp_min as i32),
            Style::default().fg(low_color),
        ),
        Span::styled(" / ", Style::default().fg(Color::DarkGray)),
        Span::styled(
            format!("{}°", day.temp_max as i32),
            Style::default().fg(high_color).add_modifier(Modifier::BOLD),
        ),
    ]));

    lines.push(Line::from(""));

    // Precipitation probability
    let precip_str = format!("{}% rain", day.precipitation_probability);
    let padding = (area.width as usize).saturating_sub(precip_str.len()) / 2;
    lines.push(Line::from(Span::styled(
        format!("{:>padding$}{}", "", precip_str, padding = padding),
        Style::default().fg(precip_color),
    )));

    // UV Index
    let uv_str = format!("UV: {} {}", day.uv_index_max as i32, uv_desc);
    let padding = (area.width as usize).saturating_sub(uv_str.len()) / 2;
    lines.push(Line::from(Span::styled(
        format!("{:>padding$}{}", "", uv_str, padding = padding),
        Style::default().fg(uv_color),
    )));

    // Wind
    let wind_str = format!("{:.0} {}", day.wind_speed_max, units.wind_speed.symbol());
    let padding = (area.width as usize).saturating_sub(wind_str.len()) / 2;
    lines.push(Line::from(Span::styled(
        format!("{:>padding$}{}", "", wind_str, padding = padding),
        Style::default().fg(Color::LightGreen),
    )));

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, area);
}

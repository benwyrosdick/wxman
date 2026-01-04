use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::config::UnitsConfig;
use crate::models::CurrentWeather;
use crate::ui::icons::{temperature_color, uv_info, wind_direction_str, WeatherCondition};

pub fn render_current_weather(
    frame: &mut Frame,
    area: Rect,
    weather: &CurrentWeather,
    units: &UnitsConfig,
) {
    let block = Block::default()
        .title(" Current Conditions ")
        .title_style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    frame.render_widget(block, area);

    // Calculate content height to center vertically
    let content_height = 11; // icon (5) + blank + temp + feels + blank + condition = 10 lines for left side
    let vertical_padding = (inner.height as usize).saturating_sub(content_height) / 2;

    // Create a centered area
    let centered_area = if vertical_padding > 0 {
        Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(vertical_padding as u16),
                Constraint::Min(content_height as u16),
                Constraint::Length(vertical_padding as u16),
            ])
            .split(inner)[1]
    } else {
        inner
    };

    // Split into left (icon + main temp) and right (details)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(20), Constraint::Min(30)])
        .split(centered_area);

    render_icon_and_temp(frame, chunks[0], weather, units);
    render_details(frame, chunks[1], weather, units);
}

fn render_icon_and_temp(
    frame: &mut Frame,
    area: Rect,
    weather: &CurrentWeather,
    units: &UnitsConfig,
) {
    let condition = WeatherCondition::from_wmo_code(weather.weather_code, weather.is_day);
    let icon = condition.icon();
    let icon_color = condition.color();

    // Convert from metric (Celsius) to user's preferred unit
    let temp = units.temperature.convert(weather.temperature);
    let feels_like = units.temperature.convert(weather.apparent_temperature);
    
    // temperature_color expects Fahrenheit, so convert if needed
    let temp_f = if units.temperature == crate::config::TemperatureUnit::Fahrenheit {
        temp
    } else {
        temp * 9.0 / 5.0 + 32.0
    };
    let temp_color = temperature_color(temp_f, true);

    let mut lines = Vec::new();

    // Add icon lines (centered)
    for line in icon.iter() {
        lines.push(Line::from(Span::styled(
            *line,
            Style::default().fg(icon_color),
        )));
    }

    // Add temperature
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        format!("{:.0}{}", temp, units.temperature.symbol()),
        Style::default()
            .fg(temp_color)
            .add_modifier(Modifier::BOLD),
    )));

    // Add feels like
    lines.push(Line::from(Span::styled(
        format!(
            "Feels {:.0}{}",
            feels_like,
            units.temperature.symbol()
        ),
        Style::default().fg(Color::Gray),
    )));

    // Add condition description
    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        condition.description(),
        Style::default().fg(icon_color),
    )));

    let paragraph = Paragraph::new(lines).alignment(Alignment::Center);
    frame.render_widget(paragraph, area);
}

fn render_details(frame: &mut Frame, area: Rect, weather: &CurrentWeather, units: &UnitsConfig) {
    let (uv_desc, uv_color) = uv_info(weather.uv_index);
    let wind_dir = wind_direction_str(weather.wind_direction);

    // Convert from metric (km/h, mm) to user's preferred units
    let wind_speed = units.wind_speed.convert(weather.wind_speed);
    let wind_gusts = units.wind_speed.convert(weather.wind_gusts);
    let precipitation = units.precipitation.convert(weather.precipitation);

    let lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Humidity:    ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}%", weather.humidity),
                Style::default().fg(Color::LightCyan),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Wind:        ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!(
                    "{:.0} {} {}",
                    wind_speed,
                    units.wind_speed.symbol(),
                    wind_dir
                ),
                Style::default().fg(Color::LightGreen),
            ),
        ]),
        Line::from(vec![
            Span::styled("Gusts:       ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{:.0} {}", wind_gusts, units.wind_speed.symbol()),
                Style::default().fg(Color::LightGreen),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Pressure:    ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{} {}", units.pressure.format(weather.pressure), units.pressure.symbol()),
                Style::default().fg(Color::LightMagenta),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Cloud Cover: ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{}%", weather.cloud_cover),
                Style::default().fg(Color::Gray),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("UV Index:    ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{:.0} ({})", weather.uv_index, uv_desc),
                Style::default().fg(uv_color),
            ),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("Precip:      ", Style::default().fg(Color::DarkGray)),
            Span::styled(
                format!("{:.2} {}", precipitation, units.precipitation.symbol()),
                Style::default().fg(Color::LightBlue),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(lines);
    frame.render_widget(paragraph, area);
}

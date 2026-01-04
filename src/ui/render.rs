use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

use crate::app::{App, AppState, UnitMenuField};
use crate::ui::chart::render_today_chart;
use crate::ui::current::render_current_weather;
use crate::ui::daily::render_daily_forecast;
use crate::ui::hourly::render_hourly_forecast;

pub fn render(frame: &mut Frame, app: &App) {
    let size = frame.area();

    // Main layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(10),    // Main content
            Constraint::Length(1),  // Footer
        ])
        .split(size);

    render_header(frame, chunks[0], app);
    render_main_content(frame, chunks[1], app);
    render_footer(frame, chunks[2], app);

    // Render overlays
    if app.show_help {
        render_help_overlay(frame, size);
    }

    if app.show_units_menu {
        render_units_menu(frame, size, app);
    }

    if app.show_location_input {
        render_location_input(frame, size, app);
    }
}

fn render_header(frame: &mut Frame, area: Rect, app: &App) {
    let location_str = app
        .location
        .as_ref()
        .map(|l| l.display_name())
        .unwrap_or_else(|| "Loading...".to_string());

    let last_updated = app
        .last_updated
        .map(|t| t.format("%l:%M %p").to_string())
        .unwrap_or_default();

    let title = format!(" WxMan - {} ", location_str);

    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            title,
            Style::default()
                .fg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(" "),
        Span::styled(
            last_updated,
            Style::default().fg(Color::DarkGray),
        ),
    ]))
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Cyan)),
    );

    frame.render_widget(header, area);
}

fn render_main_content(frame: &mut Frame, area: Rect, app: &App) {
    match &app.state {
        AppState::Loading => {
            render_loading(frame, area);
        }
        AppState::Error(msg) => {
            render_error(frame, area, msg);
        }
        AppState::Ready => {
            if let Some(weather) = &app.weather {
                // Split into top section and bottom (5-day forecast full width)
                let main_rows = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Min(20),     // Top: Current + Chart + Hourly
                        Constraint::Length(16),  // Bottom: 5-Day forecast (full width)
                    ])
                    .split(area);

                // Split top section into left (Current + Chart) and right (Hourly)
                let top_columns = Layout::default()
                    .direction(Direction::Horizontal)
                    .constraints([
                        Constraint::Percentage(55), // Left: Current + Chart
                        Constraint::Percentage(45), // Right: Hourly
                    ])
                    .split(main_rows[0]);

                // Split left column into Current and Chart
                let left_chunks = Layout::default()
                    .direction(Direction::Vertical)
                    .constraints([
                        Constraint::Min(15),     // Current conditions (fills remaining space)
                        Constraint::Length(12),  // Today's chart (fixed height)
                    ])
                    .split(top_columns[0]);

                render_current_weather(frame, left_chunks[0], &weather.current, &app.config.units);
                render_today_chart(frame, left_chunks[1], &weather.hourly, &app.config.units);
                
                // Hourly takes the full right column of top section
                render_hourly_forecast(
                    frame,
                    top_columns[1],
                    &weather.hourly,
                    &app.config.units,
                    app.hourly_scroll,
                );

                // 5-Day forecast at bottom, full width
                render_daily_forecast(frame, main_rows[1], &weather.daily, &app.config.units);
            }
        }
    }
}

fn render_loading(frame: &mut Frame, area: Rect) {
    let loading = Paragraph::new(vec![
        Line::from(""),
        Line::from(""),
        Line::from(Span::styled(
            "Loading weather data...",
            Style::default()
                .fg(Color::Yellow)
                .add_modifier(Modifier::BOLD),
        )),
    ])
    .block(Block::default().borders(Borders::ALL))
    .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(loading, area);
}

fn render_error(frame: &mut Frame, area: Rect, message: &str) {
    let error = Paragraph::new(vec![
        Line::from(""),
        Line::from(Span::styled(
            "Error",
            Style::default()
                .fg(Color::Red)
                .add_modifier(Modifier::BOLD),
        )),
        Line::from(""),
        Line::from(Span::styled(message, Style::default().fg(Color::Red))),
        Line::from(""),
        Line::from(Span::styled(
            "Press 'r' to retry",
            Style::default().fg(Color::Yellow),
        )),
    ])
    .block(Block::default().borders(Borders::ALL).border_style(Style::default().fg(Color::Red)))
    .alignment(ratatui::layout::Alignment::Center);

    frame.render_widget(error, area);
}

fn render_footer(frame: &mut Frame, area: Rect, app: &App) {
    let unit_str = app.config.units.temperature.symbol();

    let footer = Paragraph::new(Line::from(vec![
        Span::styled(" q", Style::default().fg(Color::Yellow)),
        Span::raw(" Quit  "),
        Span::styled("r", Style::default().fg(Color::Yellow)),
        Span::raw(" Refresh  "),
        Span::styled("l", Style::default().fg(Color::Yellow)),
        Span::raw(" Location  "),
        Span::styled("u", Style::default().fg(Color::Yellow)),
        Span::raw(format!(" Units ({})  ", unit_str)),
        Span::styled("↑↓", Style::default().fg(Color::Yellow)),
        Span::raw(" Scroll  "),
        Span::styled("?", Style::default().fg(Color::Yellow)),
        Span::raw(" Help"),
    ]))
    .style(Style::default().fg(Color::DarkGray));

    frame.render_widget(footer, area);
}

fn render_help_overlay(frame: &mut Frame, area: Rect) {
    // Center the help box
    let popup_width = 50;
    let popup_height = 16;
    let popup_x = (area.width.saturating_sub(popup_width)) / 2;
    let popup_y = (area.height.saturating_sub(popup_height)) / 2;

    let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height);

    // Clear the area behind the popup
    frame.render_widget(Clear, popup_area);

    let help_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  q / Esc", Style::default().fg(Color::Yellow)),
            Span::raw("     Quit application"),
        ]),
        Line::from(vec![
            Span::styled("  r", Style::default().fg(Color::Yellow)),
            Span::raw("           Refresh weather data"),
        ]),
        Line::from(vec![
            Span::styled("  l", Style::default().fg(Color::Yellow)),
            Span::raw("           Set location"),
        ]),
        Line::from(vec![
            Span::styled("  u", Style::default().fg(Color::Yellow)),
            Span::raw("           Configure units"),
        ]),
        Line::from(vec![
            Span::styled("  ↑ / k", Style::default().fg(Color::Yellow)),
            Span::raw("       Scroll hourly forecast up"),
        ]),
        Line::from(vec![
            Span::styled("  ↓ / j", Style::default().fg(Color::Yellow)),
            Span::raw("       Scroll hourly forecast down"),
        ]),
        Line::from(vec![
            Span::styled("  ?", Style::default().fg(Color::Yellow)),
            Span::raw("           Toggle this help"),
        ]),
        Line::from(""),
        Line::from(Span::styled(
            "  Press any key to close",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let help = Paragraph::new(help_text).block(
        Block::default()
            .title(" Help ")
            .title_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green)),
    );

    frame.render_widget(help, popup_area);
}

fn render_units_menu(frame: &mut Frame, area: Rect, app: &App) {
    // Center the menu box
    let popup_width = 45;
    let popup_height = 14;
    let popup_x = (area.width.saturating_sub(popup_width)) / 2;
    let popup_y = (area.height.saturating_sub(popup_height)) / 2;

    let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height);

    // Clear the area behind the popup
    frame.render_widget(Clear, popup_area);

    let selected = app.units_menu_selection;

    // Helper to create a menu row
    let make_row = |label: &str, value: &str, is_selected: bool| -> Line {
        let prefix = if is_selected { " > " } else { "   " };
        let style = if is_selected {
            Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::White)
        };
        let value_style = if is_selected {
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)
        } else {
            Style::default().fg(Color::Gray)
        };

        Line::from(vec![
            Span::styled(prefix, style),
            Span::styled(format!("{:<15}", label), style),
            Span::styled(value.to_string(), value_style),
        ])
    };

    let temp_value = app.config.units.temperature.symbol();
    let wind_value = app.config.units.wind_speed.symbol();
    let precip_value = app.config.units.precipitation.symbol();
    let pressure_value = app.config.units.pressure.symbol();

    let menu_text = vec![
        Line::from(""),
        Line::from(Span::styled(
            "  Use ↑↓ to navigate, Enter/Space to change",
            Style::default().fg(Color::DarkGray),
        )),
        Line::from(""),
        make_row("Temperature", temp_value, selected == UnitMenuField::Temperature),
        Line::from(""),
        make_row("Wind Speed", wind_value, selected == UnitMenuField::WindSpeed),
        Line::from(""),
        make_row("Precipitation", precip_value, selected == UnitMenuField::Precipitation),
        Line::from(""),
        make_row("Pressure", pressure_value, selected == UnitMenuField::Pressure),
        Line::from(""),
        Line::from(Span::styled(
            "  Press u or Esc to close and apply",
            Style::default().fg(Color::DarkGray),
        )),
    ];

    let menu = Paragraph::new(menu_text).block(
        Block::default()
            .title(" Units ")
            .title_style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Yellow)),
    );

    frame.render_widget(menu, popup_area);
}

fn render_location_input(frame: &mut Frame, area: Rect, app: &App) {
    // Center the input box
    let popup_width = 50;
    let popup_height = 10;
    let popup_x = (area.width.saturating_sub(popup_width)) / 2;
    let popup_y = (area.height.saturating_sub(popup_height)) / 2;

    let popup_area = Rect::new(popup_x, popup_y, popup_width, popup_height);

    // Clear the area behind the popup
    frame.render_widget(Clear, popup_area);

    let current_location = app
        .location
        .as_ref()
        .map(|l| l.display_name())
        .unwrap_or_else(|| "Unknown".to_string());

    let input_display = format!("{}_", app.location_input);

    let mut lines = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("  Current: ", Style::default().fg(Color::DarkGray)),
            Span::styled(current_location, Style::default().fg(Color::White)),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Enter city or zip code:", Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::raw("  "),
            Span::styled(
                input_display,
                Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
            ),
        ]),
    ];

    // Show error if any
    if let Some(error) = &app.location_error {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            format!("  {}", error),
            Style::default().fg(Color::Red),
        )));
    } else {
        lines.push(Line::from(""));
        lines.push(Line::from(Span::styled(
            "  Leave empty for auto-detect (IP)",
            Style::default().fg(Color::DarkGray),
        )));
    }

    lines.push(Line::from(""));
    lines.push(Line::from(Span::styled(
        "  Enter to confirm, Esc to cancel",
        Style::default().fg(Color::DarkGray),
    )));

    let input = Paragraph::new(lines).block(
        Block::default()
            .title(" Set Location ")
            .title_style(Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Green)),
    );

    frame.render_widget(input, popup_area);
}

mod api;
mod app;
mod config;
mod models;
mod ui;

use std::env;
use std::io;
use std::time::Duration;

use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use tokio::time::Instant;

use app::App;
use config::Config;

const REFRESH_INTERVAL: Duration = Duration::from_secs(15 * 60); // 15 minutes
const TICK_RATE: Duration = Duration::from_millis(250);
const VERSION: &str = env!("CARGO_PKG_VERSION");

#[tokio::main]
async fn main() -> Result<()> {
    if env::args().any(|arg| arg == "--version" || arg == "-V") {
        println!("wxman {VERSION}");
        return Ok(());
    }

    // Load configuration
    let config = Config::load().unwrap_or_else(|e| {
        eprintln!("Warning: Failed to load config: {}. Using defaults.", e);
        Config::default()
    });

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app
    let mut app = App::new(config);

    // Run the app
    let result = run_app(&mut terminal, &mut app).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(e) = result {
        eprintln!("Error: {}", e);
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> Result<()> {
    // Initial weather load
    if let Err(e) = app.load_weather().await {
        app.set_error(e.to_string());
    }

    let mut last_refresh = Instant::now();
    let mut last_tick = Instant::now();

    loop {
        // Draw
        terminal.draw(|frame| ui::render(frame, app))?;

        // Handle events with timeout
        let timeout = TICK_RATE
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                // Only handle key press events (not release)
                if key.kind == KeyEventKind::Press {
                    // If help is showing, any key closes it
                    if app.show_help {
                        app.show_help = false;
                        continue;
                    }

                    // If units menu is showing, handle its navigation
                    if app.show_units_menu {
                        match key.code {
                            KeyCode::Esc | KeyCode::Char('u') | KeyCode::Char('q') => {
                                app.close_units_menu();
                                // No reload needed - units are converted at display time
                            }
                            KeyCode::Up | KeyCode::Char('k') => {
                                app.units_menu_up();
                            }
                            KeyCode::Down | KeyCode::Char('j') => {
                                app.units_menu_down();
                            }
                            KeyCode::Enter
                            | KeyCode::Char(' ')
                            | KeyCode::Left
                            | KeyCode::Right => {
                                app.units_menu_toggle_selected();
                            }
                            _ => {}
                        }
                        continue;
                    }

                    // If location input is showing, handle text input
                    if app.show_location_input {
                        match key.code {
                            KeyCode::Esc => {
                                app.close_location_input();
                            }
                            KeyCode::Enter => {
                                match app.submit_location().await {
                                    Ok(true) => {
                                        // Location changed, reload weather
                                        if let Err(e) = app.load_weather().await {
                                            app.set_error(e.to_string());
                                        }
                                        last_refresh = Instant::now();
                                    }
                                    Ok(false) => {
                                        // Error shown in popup, don't close
                                    }
                                    Err(e) => {
                                        app.set_error(e.to_string());
                                        app.close_location_input();
                                    }
                                }
                            }
                            KeyCode::Backspace => {
                                app.location_input_backspace();
                            }
                            KeyCode::Char(c) => {
                                app.location_input_char(c);
                            }
                            _ => {}
                        }
                        continue;
                    }

                    match key.code {
                        KeyCode::Char('q') | KeyCode::Esc => {
                            app.should_quit = true;
                        }
                        KeyCode::Char('r') => {
                            if let Err(e) = app.load_weather().await {
                                app.set_error(e.to_string());
                            }
                            last_refresh = Instant::now();
                        }
                        KeyCode::Char('u') => {
                            app.toggle_units_menu();
                        }
                        KeyCode::Char('l') => {
                            app.open_location_input();
                        }
                        KeyCode::Up | KeyCode::Char('k') => {
                            app.scroll_hourly_up();
                        }
                        KeyCode::Down | KeyCode::Char('j') => {
                            app.scroll_hourly_down();
                        }
                        KeyCode::Char('?') => {
                            app.toggle_help();
                        }
                        _ => {}
                    }
                }
            }
        }

        if last_tick.elapsed() >= TICK_RATE {
            last_tick = Instant::now();
        }

        // Check for quit
        if app.should_quit {
            return Ok(());
        }

        // Auto-refresh
        if last_refresh.elapsed() >= REFRESH_INTERVAL {
            if let Err(e) = app.load_weather().await {
                app.set_error(e.to_string());
            }
            last_refresh = Instant::now();
        }
    }
}

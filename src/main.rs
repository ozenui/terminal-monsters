mod models;
mod ui;
mod worker;

use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use models::party::load_party;
use ratatui::prelude::*;
use std::io::{stdout, Result, Stdout};
use ui::{header_ui::render_header_ui, party_ui::render_party_ui};

fn main() -> Result<()> {
    // Set terminal tab title
    print!("\x1b]0;Terminal Monsters\x07");

    let mut terminal = setup_terminal()?;
    let mut selected_row = 0;
    let mut scroll_position = 0;
    let mut party = load_party()?;

    loop {
        // Draw UI
        terminal.draw(|frame| {
            let area = frame.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .constraints([Constraint::Min(3), Constraint::Percentage(100)].as_ref())
                .split(area);

            render_header_ui(frame, chunks[0]);
            render_party_ui(frame, chunks[1], &party, selected_row, &mut scroll_position);
        })?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Down => {
                            if selected_row < party.len().saturating_sub(1) {
                                selected_row += 1;
                                scroll_position += 1;
                            }
                        }
                        KeyCode::Up => {
                            if selected_row > 0 {
                                selected_row -= 1;
                                if selected_row < scroll_position {
                                    scroll_position = scroll_position.saturating_sub(1);
                                }
                            }
                        }
                        KeyCode::Char('r') => {
                            party = load_party()?;
                        }
                        KeyCode::Char('w') => {
                            let url = "https://github.com/enzo-rma/terminal-monsters";
                            let _ = std::process::Command::new("open").args(&[url]).status();
                        }
                        _ => {}
                    }
                }
            }
        }
    }

    teardown_terminal()?;
    Ok(())
}

/// Sets up the terminal for the application.
fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    stdout().execute(EnterAlternateScreen)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;
    Ok(terminal)
}

/// Restores the terminal to its original state.
fn teardown_terminal() -> Result<()> {
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}

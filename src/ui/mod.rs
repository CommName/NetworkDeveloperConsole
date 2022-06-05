use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{Event, read, KeyCode, KeyEvent};
use tui::layout::{Layout, Constraint};
use tui::{Terminal, Frame};
use tui::backend::{CrosstermBackend, Backend};
use core::default::Default;
use crate::app::*;

mod hosts;


pub fn draw_ui<B: Backend>(f: &mut Frame<'_, B>, app:& mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(1)
        .split(f.size());

    match app.tab_index {
        ViewState::NetworkSelection => {
            hosts::draw_hosts(f, app, rects[0]);
        },
        ViewState::NetworkView => {

        },
        ViewState::APIView => {

        }
    }
}


pub fn start_ui(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode()?;
    let stdout = std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    loop {
        terminal.draw(|f| draw_ui(f, app))?;
        if let Event::Key(key) = read()? {
            match key.code {
                KeyCode::Char('q') => {
                    break;
                }
                _ => { app.handle_key_code(key.code); }
            }
        }
    }


    disable_raw_mode()?;
    terminal.show_cursor()?;

    Ok(())
}
use tui::{
    backend::{Backend, CrosstermBackend},
    layout::{Constraint, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame, Terminal,
};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use std::sync::Arc;
use crate::store::Store;


struct App {
    state: TableState,
    store: Arc<Store>
}

impl App {
    fn new(store: Arc<Store>) -> Self {
        Self {
            state: TableState::default(),
            store
        }
    }
}

pub fn set_up_gui(store: Arc<Store>) -> std::io::Result<()> {
    let app = App::new(store);
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    run_app(&mut terminal, app)?;
    disable_raw_mode()
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> std::io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
	    if let Event::Key(key) = event::read()? {
	        match key.code {
		        KeyCode::Char('q') => return Ok(()),
		        _ => {}
	        };
	    }
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .split(frame.size());


    let header_cells = ["IP address", "Hostname"]
        .iter()
        .map(|h| Cell::from(*h).style(Style::default().fg(Color::Red)));
    let header = Row::new(header_cells);

    let destionations = app.store.get_destinations();
    let rows : Vec<Row> = destionations.into_iter().map(|destination| {
        let cell = Cell::from(destination);
        Row::new(vec!(cell))
    }).collect();

    let table = Table::new(rows)
        .header(header)
        .block(Block::default().borders(Borders::ALL).title("Destionation"))
        .widths(&[
            Constraint::Percentage(50),
            Constraint::Length(30),
            Constraint::Min(10),
        ]);
    frame.render_stateful_widget(table, rects[0], &mut app.state);
}

trait View {
    fn ui();
}

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

struct App {
    state: TableState,
    items: Vec<Vec<String>>,
}

impl App {
    fn new() -> Self {
        Self {
            state: TableState::default(),
            items: vec![
                vec!["Row11".to_string(), "Row12".to_string(), "Row13".to_string()],
                vec!["Row21".to_string(), "Row22".to_string(), "Row23".to_string()]
            ],
        }
    }
}

pub fn set_up_gui() -> std::io::Result<()> {
    enable_raw_mode()?;
    let mut stdout = std::io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);

    let mut terminal = Terminal::new(backend)?;

    let app = App::new();
    run_app(&mut terminal, app)
}

fn run_app<B: Backend>(terminal: &mut Terminal<B>, mut app: App) -> std::io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, &mut app))?;
    }
}

fn ui<B: Backend>(frame: &mut Frame<B>, app: &mut App) {
    let rects = Layout::default()
        .constraints([Constraint::Percentage(100)].as_ref())
        .margin(5)
        .split(frame.size());

    let rows = app.items.iter().map(|r| {

	let cells = r.iter().map(|c| Cell::from(*(&c.as_str())));
	Row::new(cells)
    });
    let table = Table::new(rows);
    frame.render_stateful_widget(table, rects[0], &mut app.state);
}

trait View {
    fn ui();
}

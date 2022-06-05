use tui::{
    backend::Backend,
    widgets::{Table, Row, Block, Borders},
    Frame, style::{Style, Modifier}, layout::{Rect, Constraint},
};
use super::App;

pub fn draw_hosts<B: Backend>(f: &mut Frame<'_, B>, app:& mut App, area: Rect) {

    let hosts_data = app.hosts.get_hosts();

    let cells: Vec<Row> = hosts_data.iter().map(|h| 
            Row::new(h.into_cells())
            .height(1)).collect();

    let headers = Row::new(vec!["Host", "Address", "Selected"])
        .height(1)
        .bottom_margin(1);
    

    let hosts =Table::new(cells)
        .header(headers)
        .block(Block::default().borders(Borders::ALL).title("Hosts"))
        .column_spacing(1)
        .highlight_style(Style::default().add_modifier(Modifier::BOLD))
        .highlight_symbol(">>")
        .widths(&[
            Constraint::Min(15),
            Constraint::Percentage(50),
            Constraint::Min(8),
        ]);
    
        f.render_stateful_widget(hosts, area, &mut app.hosts.table_state);
}
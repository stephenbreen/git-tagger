use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(f: &mut Frame, app: &mut App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(3)])
        .split(f.area());

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[0]);

    // Tag List
    let items: Vec<ListItem> = app
        .tags
        .iter()
        .map(|t| {
            ListItem::new(format!("{} ({})", t.name, t.date.format("%Y-%m-%d")))
        })
        .collect();

    let tag_list = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Tags"))
        .highlight_style(
            Style::default()
                .bg(Color::Blue)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(tag_list, main_chunks[0], &mut app.list_state);

    // Tag Details
    let details = if let Some(idx) = app.list_state.selected() {
        let tag = &app.tags[idx];
        let mut text = format!(
            "Name: {}\nCommit: {}\nDate: {}\nAuthor: {}\n\nMessage:\n{}",
            tag.name,
            tag.commit_id,
            tag.date,
            tag.author.as_deref().unwrap_or("N/A"),
            tag.message.as_deref().unwrap_or("N/A")
        );
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Details"))
    } else {
        Paragraph::new("No tag selected").block(Block::default().borders(Borders::ALL).title("Details"))
    };

    f.render_widget(details, main_chunks[1]);

    let footer = Paragraph::new("j/k: Navigate | d: Sort Date | s: Sort SemVer | q: Quit")
        .block(Block::default().borders(Borders::ALL).title("Help"));

    f.render_widget(footer, chunks[1]);
}

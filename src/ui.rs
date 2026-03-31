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
        .split(f.size());

    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(chunks[0]);

    // Tag List
    if app.tags.is_empty() {
        let tag_list = List::new(vec![ListItem::new("No tags found")])
            .block(Block::default().borders(Borders::ALL).title("Tags"));
        f.render_widget(tag_list, main_chunks[0]);
    } else {
        let items: Vec<ListItem> = app
            .tags
            .iter()
            .enumerate()
            .map(|(i, t)| {
                let prefix = if Some(i) == app.selected_base {
                    "[B] "
                } else {
                    "    "
                };
                let style = if t.name.ends_with("-staging") {
                    Style::default().fg(Color::Green)
                } else if t.name.ends_with("-prod") || t.name.ends_with("-production") {
                    Style::default().fg(Color::Red)
                } else {
                    Style::default()
                };
                ListItem::new(format!("{}{}({})", prefix, t.name, t.date.format("%Y-%m-%d")))
                    .style(style)
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
    };

    // Tag Details or Compare
    let details = if app.compare_mode {
        let base = &app.tags[app.selected_base.unwrap()];
        let target = &app.tags[app.list_state.selected().unwrap()];
        let mut text = format!("Comparing {} -> {}\n\nCommits:\n", base.name, target.name);
        for commit in &app.commits_between {
            text.push_str(&format!(
                "{} - {} ({})\n",
                &commit.id.to_string()[..7],
                commit.message.lines().next().unwrap_or(""),
                commit.author
            ));
        }
        Paragraph::new(text).block(Block::default().borders(Borders::ALL).title("Comparison"))
    } else if let Some(idx) = app.list_state.selected() {
        let tag = &app.tags[idx];
        let text = format!(
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

    let footer = Paragraph::new("j/k: Navigate | c: Compare | d: Sort Date | s: Sort SemVer | q: Quit")
        .block(Block::default().borders(Borders::ALL).title("Help"));

    f.render_widget(footer, chunks[1]);
}

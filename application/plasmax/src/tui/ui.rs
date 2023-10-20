use ratatui::{prelude::*, widgets::*};
use itertools::Itertools;
use super::app::{App, Mode};

pub fn ui<B: ratatui::backend::Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = create_layout(f.size());

    for (i, area) in chunks.into_iter().enumerate() {
        match i {
            1 => draw_chat_widget(f, app, area),
            2 => draw_message_box(f, app, area),
            4 => draw_chats_widget(f, app, area),
            5 => draw_new_chat_input(f, app, area),
            _ => {},
        }
    }

    draw_popup(f, app);
}

fn create_layout(area: Rect) -> Vec<Rect> {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(80), Constraint::Percentage(20)])
        .split(area);
    
    let chunks = chunks
        .iter()
        .flat_map(|area| {
            Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(1),
                    Constraint::Percentage(100),
                    Constraint::Min(3),
                ])
                .split(*area)
                .iter()
                .copied()
                .take(5)
                .collect_vec()
        })
        .collect_vec();

    chunks
}

fn draw_chats_widget<B: ratatui::backend::Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let items: Vec<ListItem> = app
        .items
        .items
        .iter()
        .map(|chat| {
            let lines = vec![Line::from(chat.user.username.clone())];
            ListItem::new(lines).style(Style::default())
        })
        .collect();

    let items = List::new(items)
        .block(Block::default().borders(Borders::ALL).title("Chats"))
        .highlight_style(
            Style::default()
                .bg(Color::LightGreen)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(items, area, &mut app.items.state);
}

fn draw_new_chat_input<B: ratatui::backend::Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let input = Paragraph::new(app.new_chat_input.input.as_str())
        .style(match app.mode {
            Mode::NewChat => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("New chat"));
    f.render_widget(input, area);
    if app.mode == Mode::NewChat {
        f.set_cursor(
            area.x + app.new_chat_input.cursor_position as u16 + 1,
            area.y + 1,
        )
    }
}

fn draw_chat_widget<B: ratatui::backend::Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    f.render_widget(Block::default().borders(Borders::ALL).title(""), area);
}

fn draw_message_box<B: ratatui::backend::Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let input = Paragraph::new(app.message_input.input.as_str())
        .style(match app.mode {
            Mode::Message => Style::default().fg(Color::Yellow),
            _ => Style::default(),
        })
        .block(Block::default().borders(Borders::ALL).title("Message"));
    f.render_widget(input, area);
    if app.mode == Mode::Message {
        f.set_cursor(
            area.x + app.message_input.cursor_position as u16 + 1,
            area.y + 1,
        )
    }
}


fn draw_popup<B: ratatui::backend::Backend>(f: &mut Frame<B>, app: &App) {
    match app.items.get() {
        Some(cur) => {
            let block = Block::default().title(format!("Current: {}", cur)).borders(Borders::ALL);
            let area = centered_rect(40, 10, f.size());
            f.render_widget(Clear, area);
            f.render_widget(block, area);
        },
        None => {},
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

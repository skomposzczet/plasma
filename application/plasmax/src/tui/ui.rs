use ratatui::{prelude::*, widgets::*};
use itertools::Itertools;
use super::{app::App, tools::Mode};

pub fn ui<B: ratatui::backend::Backend>(f: &mut Frame<B>, app: &mut App) {
    let chunks = create_layout(f.size());

    for (i, area) in chunks.into_iter().enumerate() {
        match i {
            0 => print_small_help(f, app, area),
            1 => draw_chat_widget(f, app, area),
            2 => draw_message_box(f, app, area),
            4 => draw_chats_widget(f, app, area),
            5 => draw_new_chat_input(f, app, area),
            _ => {},
        }
    }
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
        .block(
            Block::default()
            .borders(Borders::ALL)
            .title("Chats")
            .border_style(match app.mode {
                Mode::BrowseChats => Style::default().fg(Color::LightGreen),
                _ => Style::default(),
            })
        )
        .highlight_style(
            Style::default()
                .bg(Color::Magenta)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol(">> ");

    f.render_stateful_widget(items, area, &mut app.items.state);
}

fn draw_new_chat_input<B: ratatui::backend::Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let input = Paragraph::new(app.new_chat_input.input.as_str())
        .style(match app.mode {
            Mode::NewChat => Style::default().fg(Color::LightGreen),
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

fn print_small_help<B: ratatui::backend::Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let text = app.get_small_help();
    let help_message = Paragraph::new(text);
    f.render_widget(help_message, area);
}

fn draw_chat_widget<B: ratatui::backend::Backend>(f: &mut Frame<B>, app: &mut App, area: Rect) {
    let text = app.messages_buffer.text();
    let scroll = app.calculate_scroll(area.height, text.height() as u16);
    let title = match app.items.get() {
        None => String::from(""),
        Some(chat) => format!("Chat with {}", chat.user.username),
    };

    let paragraph = Paragraph::new(text)
        .block(
            Block::new()
            .title(title)
            .borders(Borders::ALL)
            .border_style(match app.mode {
                Mode::ChatScroll => Style::default().fg(Color::LightGreen),
                _ => Style::default(),
            })
        )
        .scroll((scroll, 0));
    f.render_widget(paragraph, area);

    if scroll == 0 {
        app.messages_buffer.scroll_block();
    } else {
        app.messages_buffer.scroll_unblock();
    }
}

fn draw_message_box<B: ratatui::backend::Backend>(f: &mut Frame<B>, app: &App, area: Rect) {
    let input = Paragraph::new(app.message_input.input.as_str())
        .style(match app.mode {
            Mode::Message => Style::default().fg(Color::LightGreen),
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

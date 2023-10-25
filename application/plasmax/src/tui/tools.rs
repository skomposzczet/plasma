use std::fmt::Display;
use ratatui::{widgets::ListState, text::{Span, Line, Text}, style::{Color, Style, Modifier}};

pub struct StatefulList<T> {
    pub state: ListState,
    pub items: Vec<T>,
    current: Option<usize>,
}

impl<T> StatefulList<T> {
    pub fn with_items(items: Vec<T>) -> StatefulList<T> {
        StatefulList {
            state: ListState::default(),
            items,
            current: None,
        }
    }

    pub fn select(&mut self) -> bool {
        let next = self.state.selected();
        if next == self.current {
            return false;
        }
        self.current = self.state.selected();
        return true;
    }

    pub fn get(&self) -> Option<&T> {
        match self.current {
            None => None,
            Some(idx) => Some(&self.items[idx]),
        }
    }

    pub fn next(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn previous(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    pub fn unselect(&mut self) {
        self.state.select(None);
    }
}

pub struct UserInput {
    pub cursor_position: usize,
    pub input: String,
}

impl UserInput {
    pub fn new() -> Self {
        UserInput {
            cursor_position: 0,
            input: String::new(),
        }
    }
    
    pub fn submit(&mut self) -> String {
        let tmp = self.input.clone();
        self.input.clear();
        self.reset_cursor();
        tmp
    }

    pub fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    pub fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    pub fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position, new_char);
        self.move_cursor_right();
    }

    pub fn delete_char(&mut self) {
        let is_not_cursor_leftmost = self.cursor_position != 0;
        if is_not_cursor_leftmost {

            let current_index = self.cursor_position;
            let from_left_to_current_index = current_index - 1;

            let before_char_to_delete = self.input.chars().take(from_left_to_current_index);
            let after_char_to_delete = self.input.chars().skip(current_index);

            self.input = before_char_to_delete.chain(after_char_to_delete).collect();
            self.move_cursor_left();
        }
    }

    fn clamp_cursor(&self, new_cursor_pos: usize) -> usize {
        new_cursor_pos.clamp(0, self.input.len())
    }

    fn reset_cursor(&mut self) {
        self.cursor_position = 0;
    }
}

#[derive(PartialEq)]
pub enum Mode {
    Normal,
    BrowseChats,
    NewChat,
    Message,
    ChatScroll,
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Mode::Normal => write!(f, "Normal"),
            Mode::BrowseChats => write!(f, "Browse"),
            Mode::NewChat | Mode::Message => write!(f, "Input"),
            Mode::ChatScroll => write!(f, "Scroll"),
        }
    }
}

struct Message {
    username: String,
    content: String,
}

pub struct MessagesBuffer {
    me: String,
    messages: Vec<Message>,
    scroll_offset: u16,
    scroll_up_block: bool,
}

impl MessagesBuffer {
    pub fn new(me: String) -> Self {
        MessagesBuffer {
            me,
            messages: Vec::new(),
            scroll_offset: 0,
            scroll_up_block: false,
        }
    }

    pub fn push(&mut self, username: &str, message: &str) {
        let m = Message {
            username: String::from(username),
            content: String::from(message),
        };
        self.messages.push(m);
    }

    pub fn text(&self) -> Text {
        let lines: Vec<Line> = self.messages
            .iter()
            .map(|m| {
                self.to_line(m)
            })
            .collect();
        Text::from(lines)
    }

    fn to_line(&self, message: &Message) -> Line {
        let color = if self.me == message.username { Color::Green } else { Color::Red };
        let span = Span::styled(
            format!("@{}:", message.username),
            Style::new()
            .fg(color)
            .add_modifier(Modifier::BOLD),
        );
        let span2 = Span::raw(message.content.clone());
        let line = Line::from(vec![span, span2]);
        line
    }

    pub fn scroll_get(&self) -> u16 {
        self.scroll_offset
    }

    pub fn scroll_up(&mut self) {
        if self.scroll_up_block {
            return;
        }
        self.scroll_offset = self.scroll_offset.saturating_add(1);
    }

    pub fn scroll_down(&mut self) {
        self.scroll_offset = self.scroll_offset.saturating_sub(1);
    }

    pub fn scroll_block(&mut self) {
        self.scroll_up_block = true;
    }
    
    pub fn scroll_unblock(&mut self) {
        self.scroll_up_block = false;
    }

    pub fn scroll_reset(&mut self) {
        self.scroll_offset = 0;
    }
}

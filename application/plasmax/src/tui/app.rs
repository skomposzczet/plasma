use crossterm::event::KeyCode;
use ratatui::widgets::ListState;

use crate::chats::Chat;

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

    pub fn show(&mut self) {
        self.current = match self.current {
            Some(_) => None,
            None => self.state.selected(),
        }
    }

    pub fn get(&self) -> Option<usize> {
        self.current
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

    fn move_cursor_left(&mut self) {
        let cursor_moved_left = self.cursor_position.saturating_sub(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_left);
    }

    fn move_cursor_right(&mut self) {
        let cursor_moved_right = self.cursor_position.saturating_add(1);
        self.cursor_position = self.clamp_cursor(cursor_moved_right);
    }

    fn enter_char(&mut self, new_char: char) {
        self.input.insert(self.cursor_position, new_char);
        self.move_cursor_right();
    }

    fn delete_char(&mut self) {
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
}

pub struct App {
    pub mode: Mode,
    pub items: StatefulList<Chat>,
    pub new_chat_input: UserInput,
    pub message_input: UserInput,
}

impl App {
    pub fn new(chats: Vec<Chat>) -> App {
        App {
            mode: Mode::Normal,
            items: StatefulList::with_items(chats),
            new_chat_input: UserInput::new(),
            message_input: UserInput::new(),
        }
    }

    pub fn handle_evt(&mut self, key: KeyCode) -> bool {
        match self.mode {
            Mode::Normal => self.handle_evt_normal(key),
            Mode::BrowseChats => self.handle_evt_browse_chats(key),
            Mode::NewChat | Mode::Message => self.handle_evt_input(key),
        }
    }

    fn handle_evt_browse_chats(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Left => self.items.unselect(),
            KeyCode::Down => self.items.next(),
            KeyCode::Up => self.items.previous(),
            KeyCode::Right => self.items.show(),
            _ => return false,
        }
        return true;
    }

    fn handle_evt_normal(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('b') => self.mode = Mode::BrowseChats,
            KeyCode::Char('n') => self.mode = Mode::NewChat,
            KeyCode::Char('m') => self.mode = Mode::Message,
            _ => return false,
        }
        return true;
    }

    fn handle_evt_input(&mut self, key: KeyCode) -> bool {
        let input = match self.mode {
            Mode::NewChat => &mut self.new_chat_input,
            Mode::Message => &mut self.message_input,
            _ => {
                return false;
            }
        };

        match key {
            KeyCode::Enter => {
                input.submit();
            },
            KeyCode::Char(to_insert) => {
                input.enter_char(to_insert);
            }
            KeyCode::Backspace => {
                input.delete_char();
            }
            KeyCode::Left => {
                input.move_cursor_left();
            }
            KeyCode::Right => {
                input.move_cursor_right();
            }
            _ => {
                return false;
            }
        }
        return true;
    }
}

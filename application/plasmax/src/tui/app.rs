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
}

impl App {
    pub fn new(chats: Vec<Chat>) -> App {
        App {
            mode: Mode::Normal,
            items: StatefulList::with_items(chats),
        }
    }

    pub fn handle_evt(&mut self, key: KeyCode) -> bool {
        match self.mode {
            Mode::Normal => self.handle_evt_normal(key),
            Mode::BrowseChats => self.handle_evt_browse_chats(key),
            Mode::NewChat => self.handle_evt_new_chat(key),
            Mode::Message => self.handle_evt_message(key),
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

    fn handle_evt_new_chat(&mut self, key: KeyCode) -> bool {
        todo!();
    }

    fn handle_evt_message(&mut self, key: KeyCode) -> bool {
        todo!();
    }
}

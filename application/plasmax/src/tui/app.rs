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

pub struct App {
    pub items: StatefulList<Chat>,
}

impl App {
    pub fn new(chats: Vec<Chat>) -> App {
        App {
            items: StatefulList::with_items(chats),
        }
    }
}

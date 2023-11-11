use crossterm::event::KeyCode;
use crate::{api::{Api, ws::{ThreadComm, Ws, WsMessage}}, account::{Account, Authorized}, error::PlasmaError, chats::Chat};
use super::tools::{Mode, StatefulList, UserInput, MessagesBuffer};

pub struct App {
    pub api: Api,
    pub account: Account<Authorized>,
    pub mode: Mode,
    pub items: StatefulList<Chat>,
    pub new_chat_input: UserInput,
    pub message_input: UserInput,
    pub messages_buffer: MessagesBuffer,
    pub comms: ThreadComm<WsMessage>,
}

impl App {
    pub async fn new(api: Api, account: Account<Authorized>) -> Result<App, PlasmaError> {
        let chats = account.chats(&api).await?.chats;
        let un = account.username().clone();
        let ws = Ws::new("ws://localhost:8000/chat", account.token());
        let comms = ws.run().await;
        let app = App {
            api,
            account,
            mode: Mode::BrowseChats,
            items: StatefulList::with_items(chats),
            new_chat_input: UserInput::new(),
            message_input: UserInput::new(),
            messages_buffer: MessagesBuffer::new(un),
            comms,
        };
        Ok(app)
    }

    pub async fn on_tick(&mut self) {
        let message = match self.comms.receiver.try_recv() {
            Ok(mess) => mess,
            Err(_) => return,
        };
        self.messages_buffer.push("other", &message.content);
    }

    pub fn calculate_scroll(&self, area_height: u16, text_height: u16) -> u16 {
        let scroll = if text_height < area_height - 2 { 0 } else { text_height + 2 - area_height };
        let scroll = scroll.saturating_sub(self.messages_buffer.scroll_get());
        scroll
    }

    pub fn get_small_help(&self) -> String {
        format!("Mode: {}", self.mode)
    }

    pub async fn handle_evt(&mut self, key: KeyCode) -> bool {
        match self.mode {
            Mode::Normal => self.handle_evt_normal(key),
            Mode::BrowseChats => self.handle_evt_browse_chats(key).await,
            Mode::NewChat | Mode::Message => self.handle_evt_input(key).await,
            Mode::ChatScroll => self.handle_evt_scroll(key),
        }
    }

    async fn handle_evt_browse_chats(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Left | KeyCode::Char('h') => self.items.unselect(),
            KeyCode::Down | KeyCode::Char('j')  => self.items.next(),
            KeyCode::Up | KeyCode::Char('k')  => self.items.previous(),
            KeyCode::Enter => self.init_message_buffer().await,
            _ => return false,
        }
        return true;
    }

    async fn init_message_buffer(&mut self) {
        let changed = self.items.select();
        if !changed {
            return;
        }
        let chat = self.items.get().unwrap();
        self.account
            .ensure_secret(&self.api, &chat.id, &chat.user.username)
            .await
            .unwrap();
        let oid = self.account.id();
        self.messages_buffer = MessagesBuffer::new(self.account.username().clone());
        for message in self.account.messages(&self.api, &chat.id).await.unwrap().iter() {
            let username = match message.sender_id == *oid {
                true => self.account.username().clone(),
                false => self.items.get().unwrap().user.username.clone(),
            };
            self.messages_buffer.push(&username, &message.message);
        }
    }

    fn handle_evt_normal(&mut self, key: KeyCode) -> bool {
        self.mode = match key {
            KeyCode::Char('b') => Mode::BrowseChats,
            KeyCode::Char('n') => Mode::NewChat,
            KeyCode::Char('s') => Mode::ChatScroll,
            KeyCode::Char('m') => {
                match self.items.get() {
                    Some(_) => Mode::Message,
                    None => Mode::Normal,
                }
            }
            _ => return false,
        };
        return true;
    }

    async fn handle_evt_input(&mut self, key: KeyCode) -> bool {
        let input = match self.mode {
            Mode::NewChat => &mut self.new_chat_input,
            Mode::Message => &mut self.message_input,
            _ => {
                return false;
            }
        };

        match key {
            KeyCode::Enter => {
                self.submit().await;
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

    async fn submit(&mut self) {
        match self.mode {
            Mode::NewChat => {
                let username = self.new_chat_input.submit();
                match self.account.chat(&self.api, &username).await {
                    Ok(_) => {
                        let chats = self.account.chats(&self.api).await.unwrap().chats;
                        self.items = StatefulList::with_items(chats);
                    },
                    Err(_) => {
                        todo!();
                    }
                }
            },
            Mode::Message => {
                let message = self.message_input.submit();
                if message.is_empty() {
                    return;
                }
                let current_chat = match self.items.get() {
                    Some(chat) => chat,
                    None => return,
                };
                let message = WsMessage {
                    chat_id: current_chat.id.to_string(),
                    sender_id: self.account.id().clone().to_string(),
                    content: message,
                };
                self.messages_buffer.push(self.account.username(), &message.content);
                self.comms.sender.send(message).await.unwrap();
            },
            _ => {},
        }
    }

    fn handle_evt_scroll(&mut self, key: KeyCode) -> bool {
        match key {
            KeyCode::Char('j') | KeyCode::Down => {
                self.messages_buffer.scroll_down();
            }
            KeyCode::Char('k') | KeyCode::Up =>  {
                self.messages_buffer.scroll_up();
            }
            KeyCode::Char('q') => {
                self.messages_buffer.scroll_reset();
            }
            _ => {
                return false;
            }
        }
        return true;
    }
}

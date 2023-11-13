use bson::oid::ObjectId;
use crossterm::event::KeyCode;
use crate::{api::{Api, ws::{ThreadComm, Ws, WsMessage}}, account::{Account, Authorized}, error::PlasmaError, chats::Chat, cipher::Cipher};
use super::tools::{Mode, StatefulList, UserInput, MessagesBuffer, ErrorMessage};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct App {
    pub api: Api,
    pub account: Account<Authorized>,
    pub mode: Mode,
    pub items: StatefulList<Chat>,
    pub new_chat_input: UserInput,
    pub message_input: UserInput,
    pub messages_buffer: MessagesBuffer,
    pub comms: ThreadComm<WsMessage>,
    pub cipher: Option<Cipher>,
    pub error_message: ErrorMessage,
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
            cipher: None,
            error_message: ErrorMessage::default(),
        };
        Ok(app)
    }

    pub async fn on_tick(&mut self) {
        if let Err(e) = self.on_tick_impl().await {
            self.error_message.set(&format!("{}", e));
        }
    }
    pub async fn on_tick_impl(&mut self) -> Result<(), PlasmaError> {
        let message = match self.comms.receiver.try_recv() {
            Ok(mess) => mess,
            Err(_) => return Ok(()),
        };
        let decrypted = self.cipher
            .as_ref()
            .expect("Cipher should be some if messages are read")
            .decrypt(&message.content, message.timestamp)?;
        self.messages_buffer.push("other", &decrypted);
        Ok(())
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
        match self.handle_evt_impl(key).await {
            Ok(res) => res,
            Err(e) => {
                self.error_message.set(&format!("{}", e));
                false
            }
        }
    }

    async fn handle_evt_impl(&mut self, key: KeyCode) -> Result<bool, PlasmaError> {
        if self.error_message.is_err() {
            self.error_message.clear();
            return Ok(true);
        }
        match self.mode {
            Mode::Normal => self.handle_evt_normal(key),
            Mode::BrowseChats => self.handle_evt_browse_chats(key).await,
            Mode::NewChat | Mode::Message => self.handle_evt_input(key).await,
            Mode::ChatScroll => self.handle_evt_scroll(key),
        }
    }

    async fn handle_evt_browse_chats(&mut self, key: KeyCode) -> Result<bool, PlasmaError> {
        match key {
            KeyCode::Left | KeyCode::Char('h') => self.items.unselect(),
            KeyCode::Down | KeyCode::Char('j')  => self.items.next(),
            KeyCode::Up | KeyCode::Char('k')  => self.items.previous(),
            KeyCode::Enter => self.init_message_buffer().await?,
            _ => return Ok(false),
        }
        return Ok(true);
    }

    async fn init_message_buffer(&mut self) -> Result<(), PlasmaError> {
        let changed = self.items.select();
        if !changed {
            return Ok(());
        }
        let chat = self.items
            .get()
            .expect("Has value because select returns true");
        self.account
            .ensure_secret(&self.api, &chat.id, &chat.user.username)
            .await?;
        let cipher = self.account
            .get_cipher(&chat.user.username)?;
        self.cipher = Some(cipher);
        let oid = self.account.id();
        self.messages_buffer = MessagesBuffer::new(self.account.username().clone());
        for message in self.account.messages(&self.api, &chat.id).await?.iter() {
            let username = match message.sender_id == *oid {
                true => self.account.username().clone(),
                false => chat.user.username.clone(),
            };
            let decrypted = self.cipher
                .as_ref()
                .expect("Cipher should be some if messages are read")
                .decrypt(&message.message, message.timestamp)?;
            self.messages_buffer.push(&username, &decrypted);
        }
        Ok(())
    }

    fn handle_evt_normal(&mut self, key: KeyCode) -> Result<bool, PlasmaError> {
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
            _ => return Ok(false),
        };
        return Ok(true);
    }

    async fn handle_evt_input(&mut self, key: KeyCode) -> Result<bool, PlasmaError> {
        let input = match self.mode {
            Mode::NewChat => &mut self.new_chat_input,
            Mode::Message => &mut self.message_input,
            _ => {
                return Ok(false);
            }
        };

        match key {
            KeyCode::Enter => {
                self.submit().await?;
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
                return Ok(false);
            }
        }
        return Ok(true);
    }

    async fn submit(&mut self) -> Result<(), PlasmaError> {
        match self.mode {
            Mode::NewChat => self.submit_new_chat().await?,
            Mode::Message => self.submit_message().await?,
            _ => {},
        }
        Ok(())
    }

    async fn submit_new_chat(&mut self) -> Result<(), PlasmaError> {
        let username = self.new_chat_input.submit();
        self.account.chat(&self.api, &username).await?;
        let chats = self.account.chats(&self.api).await?.chats;
        self.items = StatefulList::with_items(chats);
        Ok(())
    }

    async fn submit_message(&mut self) -> Result<(), PlasmaError> {
        let message = self.message_input.submit();
        if message.is_empty() {
            return Ok(());
        }
        let current_chat = self.items
            .get()
            .expect("Not possible to write message when no chat selected");
        let ws_message = self.make_message(&message, &current_chat.id)?;
        self.messages_buffer.push(self.account.username(), &message);
        self.comms.sender.send(ws_message).await.unwrap();
        Ok(())
    }

    fn make_message(&self, message: &str, chat_id: &ObjectId) -> Result<WsMessage, PlasmaError> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_micros() as u64;
        let encrypted = self.cipher
            .as_ref()
            .expect("Cipher should be some if messages are read")
            .encrypt(&message, timestamp)?;
        let ws_message = WsMessage {
            chat_id: chat_id.to_string(),
            sender_id: self.account.id().to_string(),
            content: encrypted,
            timestamp,
        };
        Ok(ws_message)
    }

    fn handle_evt_scroll(&mut self, key: KeyCode) -> Result<bool, PlasmaError> {
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
                return Ok(false);
            }
        }
        return Ok(true);
    }
}

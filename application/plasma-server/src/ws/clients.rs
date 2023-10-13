use std::collections::HashMap;
use tokio::sync::mpsc;
use warp::{filters::ws::Message, reject::Rejection};

type Client = mpsc::UnboundedSender<Message>;

pub struct Clients {
    client_map: HashMap<String, Client>,
}

impl Clients {
    pub fn new() -> Self {
        Clients {
            client_map: HashMap::new(),
        }
    }

    pub fn get_client(&mut self, user_id: &str) -> Option<&Client> {
        self.client_map.get(user_id)
    }

    pub fn add_client(&mut self, user_id: String, client: Client) {
        self.client_map.insert(user_id, client);
    }

    pub fn remove_client(&mut self, user_id: &str) {
        self.client_map.remove(user_id);
    }
}

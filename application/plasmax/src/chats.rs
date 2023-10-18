use std::iter::zip;
use bson::oid::ObjectId;
use crate::api::response;

#[derive(Debug, Clone)]
pub struct UserHandle {
    pub id: ObjectId,
    pub username: String,
}

#[derive(Debug, Clone)]
pub struct Chat {
    pub id: ObjectId,
    pub user: UserHandle,
}

#[derive(Debug, Clone)]
pub struct Chats {
    pub chats: Vec<Chat>,
}

pub fn get_non_user_id(users: &Vec<ObjectId>, userid: &ObjectId) -> Option<ObjectId> {
    for id in users {
        if id != userid {
            return Some(id.clone());
        }
    }
    None
}

impl Chats {
    pub fn new(chats: Vec<response::Chat>, usernames: Vec<String>, userid: ObjectId) -> Self {
        let cs = zip(chats, usernames)
            .map(|c| {
                Chat {
                    id: c.0.id.clone(),
                    user: UserHandle {
                        id: get_non_user_id(&c.0.users, &userid).unwrap(),
                        username: c.1.clone(),
                    }
                }
            })
            .collect();

        Chats { chats: cs }
    }
}

impl IntoIterator for Chats {
    type Item = Chat;
    type IntoIter = std::vec::IntoIter<Self::Item>;
    fn into_iter(self) -> Self::IntoIter {
        self.chats.into_iter()
    }
}

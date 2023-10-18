use serde::Serialize;
use bson::oid::ObjectId;

#[derive(Serialize)]
pub struct RegisterBody {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginBody {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct FindBody {
    pub id: Option<ObjectId>,
    pub email: Option<String>,
    pub username: Option<String>,
}

impl FindBody {
    pub fn id(id: ObjectId) -> Self {
        FindBody { id: Some(id), email: None, username: None, }
    }
    pub fn username(username: String) -> Self {
        FindBody { id: None, email: None, username: Some(username) }
    }
}

#[derive(Serialize)]
pub struct ChatBody {
    pub member: String,
}

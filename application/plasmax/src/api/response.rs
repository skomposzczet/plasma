use bson::oid::ObjectId;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct OkResponse<T> {
    pub data: T,
}

#[derive(Deserialize)]
pub struct RegisterResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub jwtoken: String,
}

#[derive(Deserialize)]
pub struct DashboardResponse {
    pub username: String,
}

#[derive(Deserialize)]
pub struct User {
    pub id: ObjectId,
    pub email: String,
    pub username: String,
}

#[derive(Deserialize)]
pub struct FindResponse {
    pub user: User,
}

#[derive(Deserialize)]
pub struct Chat {
    #[serde(rename = "_id")]
    pub id: ObjectId,
    pub users: Vec<ObjectId>,
}

#[derive(Deserialize)]
pub struct ChatsResponse {
    pub chats: Vec<Chat>,
}

#[derive(Deserialize)]
pub struct ChatResponse {
    pub chatid: ObjectId,
}

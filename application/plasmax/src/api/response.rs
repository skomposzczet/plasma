use bson::oid::ObjectId;
use serde::Deserialize;
use x3dh::handshake;

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

#[derive(Deserialize)]
pub struct Message {
    #[serde(rename = "_id")]
    pub id: Option<ObjectId>,
    pub chat_id: ObjectId,
    pub sender_id: ObjectId,
    pub message: Vec<u8>,
    pub timestamp: u64,
}

#[derive(Deserialize)]
pub struct MessagesResponse {
    pub messages: Vec<Message>,
}

#[derive(Deserialize)]
pub struct SendBundleResponse {
    pub bundle: String,
}

#[derive(Deserialize)]
pub struct PeerBundleResponse {
    pub bundle: handshake::PeerBundleBinary,
}

#[derive(Deserialize)]
pub struct InitialMessageResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct GetInitialMesssageResponse {
    pub message: Option<handshake::InitialMessageBinary>,
}

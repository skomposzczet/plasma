use serde::Serialize;

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

use serde::Serialize;

#[derive(Serialize)]
pub struct LoginBody {
    pub email: String,
    pub password: String,
}

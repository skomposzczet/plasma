use serde::Deserialize;

#[derive(Deserialize)]
pub struct OkResponse<T> {
    pub data: T,
}

#[derive(Deserialize)]
pub struct LoginResponse {
    pub jwtoken: String,
}

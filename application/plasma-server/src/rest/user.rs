use bson::oid::ObjectId;
use warp::{Filter, Rejection, reply::Json};
use std::sync::Arc;
use crate::error::AuthorizationError;
use crate::model;
use crate::model::{Db, user::User};
use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::security::{hash::hashed_password, token};
use crate::rest::json_response;
use crate::server::with_auth;

#[derive(Deserialize, Debug)]
struct RegisterBody {
    email: String,
    username: String,
    password: String
}

#[derive(Deserialize, Debug)]
struct LoginBody {
    email: String,
    password: String
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub jwtoken: String,
}

#[derive(Deserialize)]
struct FindBody {
    id: Option<ObjectId>,
    email: Option<String>,
    username: Option<String>,
}

#[derive(Serialize)]
struct UserResponse {
    id: Option<ObjectId>,
    email: String,
    username: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        UserResponse {
            id: user.id,
            email: user.email,
            username: user.username,
        }
    }
}

pub fn account_paths(db: Arc<Db>) -> impl Filter<Extract = (impl warp::Reply,), Error = Rejection> + Clone {
    let with_db = warp::any()
        .map(move || db.clone());

    let register = warp::path("register")
        .and(warp::path::end())
        .and(warp::post())
        .and(with_db.clone())
        .and(warp::body::json())
        .and_then(register_handle);

    let login = warp::path("login")
        .and(warp::path::end())
        .and(warp::post())
        .and(with_db.clone())
        .and(warp::body::json())
        .and_then(login_handle);

    let dashboard = warp::path("dashboard")
        .and(warp::path::end())
        .and(warp::get())
        .and(with_db.clone())
        .and(with_auth())
        .and_then(dashboard_handle);

    let find = warp::path("user")
        .and(warp::path::end())
        .and(warp::post())
        .and(with_db.clone())
        .and(warp::body::json())
        .and_then(find_handle);

    register
        .or(login)
        .or(dashboard)
        .or(find)
}

async fn login_handle(db: Arc<Db>, body: LoginBody) -> Result<Json, Rejection> {
    let user = User::get_by_email(&db, &body.email).await
        .map_err(|_| AuthorizationError::InvalidCredentials("email"))?;

    if !user.password_matches(&hashed_password(&body.password)) {
        return Err(AuthorizationError::InvalidCredentials("password").into());
    }
    let token = token::create_jwt(&user)?;

    json_response(&LoginResponse {jwtoken: token})
}

async fn register_handle(db: Arc<Db>, body: RegisterBody) -> Result<Json, Rejection> {
    let is_unique = is_unique_email(&db, &body.email).await?;
    if !is_unique {
        return Err(model::Error::NotUnique("email").into());
    }

    let new_user = User::new(
        &body.email,
        &body.username,
        &hashed_password(&body.password)
    );
    User::add_to_db(&db, &new_user).await?;

    let content = json!({
        "message": "success",
    });
    json_response(&content)
}

async fn is_unique_email(db: &Db, email: &String) -> Result<bool, model::Error> {
    match User::get_by_email(db, &email).await {
        Ok(_) => Ok(false),
        Err(err) => {
            match err {
                model::Error::NoUserWithSuchEmail => Ok(true),
                _ => Err(err)
            }
        }
    }
}

async fn dashboard_handle(db: Arc<Db>, id: String) -> Result<Json, Rejection> {
    let user = User::get_by_id(&db, &id).await?;

    let content = json!({
        "username": user.username()
    });
    json_response(&content)
}

async fn find_handle(db: Arc<Db>, body: FindBody) -> Result<Json, Rejection> {
    let user = {
        if body.id.is_some() {
            Some(User::get_by_id(&db, &format!("ObjectId(\"{}\")",body.id.unwrap())).await?)
        } else if body.username.is_some() {
            Some(User::get_by_username(&db, &body.username.unwrap()).await?)
        } else if body.email.is_some() {
            Some(User::get_by_email(&db, &body.email.unwrap()).await?)
        } else {
            None
        }
    };

    let user = match user {
        Some(u) => Some(UserResponse::from(u)),
        None => None,
    };

    let content = json!({
        "user": user
    });
    json_response(&content)
}

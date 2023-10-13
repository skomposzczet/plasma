mod web_error;

use std::{sync::Arc, convert::Infallible};
use serde_json::json;
use warp::{Rejection, Filter, hyper::HeaderMap, http::HeaderValue, Reply};
use crate::ClientsHandle;
use crate::{error::AuthorizationError, ws, rest, error};
use crate::{security::token::{jwt_from_header, decode_jwt}, model::Db};
use web_error::WebErrorMessage;

pub fn routes(db: Arc<Db>, clients: ClientsHandle) -> impl Filter<Extract = (impl Reply,), Error = Infallible> + Clone {
    rest::rest_routes(db.clone())
        .or(ws::ws_paths(db.clone(), clients.clone()))
        .recover(handle_rejection)
}

async fn handle_rejection(err: Rejection) -> Result<impl Reply, Infallible> {
    error!("ERROR - {:?}", err);

    let error_message = match err.find::<WebErrorMessage>() {
        Some(err) => err.clone(),
        None => WebErrorMessage::unknown()
    };
    info!("{}", error_message.message);

    let result = json!({
        "error": error_message.kind
    });
    let result = warp::reply::json(&result);

    Ok(warp::reply::with_status(result, error_message.status_code))
}

pub fn with_auth() -> impl Filter<Extract = (String,), Error = Rejection> + Clone {
    warp::header::headers_cloned()
        .and_then(auth)
}

async fn auth(auth_header: HeaderMap<HeaderValue>) -> Result<String, warp::Rejection> {
    let token = jwt_from_header(&auth_header)
        .ok_or(AuthorizationError::MissingAuthHeader)?;

    let token_data = match decode_jwt(&token) {
        Ok(data) => data,
        Err(err) => {
            match err {
                error::Error::JWTokenError(_) => return Err(AuthorizationError::from(err).into()),
                _ => return Err(err.into())
            }
        }
    };

    Ok(token_data.claims.sub())
}

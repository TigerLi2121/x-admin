use crate::common::db::get_pool;
use crate::common::jwt;
use crate::common::res::R;
use crate::model::user::User;
use axum::extract::Request;
use axum::http::{header, StatusCode};
use axum::middleware::Next;
use axum::response::IntoResponse;
use serde_json::Value;
use sqlx::Error;

pub async fn auth(mut req: Request, next: Next) -> Result<impl IntoResponse, (StatusCode, String)> {
    let logout = Ok(R::<Value>::code(886, String::from("login user")).into_response());
    let token = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|auth_header| auth_header.to_str().ok())
        .and_then(|auth_value| {
            if auth_value.starts_with("Bearer ") {
                Some(auth_value[7..].to_owned())
            } else {
                None
            }
        });
    if token.is_none() {
        return logout;
    }
    let uid = jwt::get_uid(token.unwrap());
    if uid == 0 {
        return logout;
    }

    let user: Result<User, Error> = sqlx::query_as("SELECT * FROM user WHERE id = ?")
        .bind(uid)
        .fetch_one(get_pool().unwrap())
        .await;
    if user.is_err() {
        return logout;
    }
    req.extensions_mut().insert(user.unwrap());
    Ok(next.run(req).await)
}

use crate::common::req::Page;
use crate::common::res::{R, RP};
use crate::model::role;
use crate::model::role::Role;
use axum::extract::Query;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::Value;

pub fn router() -> Router {
    Router::new().route("/", get(page).post(sou).delete(del))
}

pub async fn page(Query(p): Query<Page>) -> RP<Vec<Role>> {
    role::page(p).await.unwrap_or(RP::ok(0, vec![]))
}

pub async fn sou(Json(m): Json<Role>) -> R<Value> {
    match role::sou(m).await {
        Ok(_) => R::ok(),
        Err(e) => R::err_msg(e.to_string()),
    }
}

pub async fn del(Json(ids): Json<Vec<u64>>) -> R<Value> {
    match role::del(ids).await {
        Ok(_) => R::ok(),
        Err(e) => R::err_msg(e.to_string()),
    }
}

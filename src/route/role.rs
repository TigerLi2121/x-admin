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
    role::page(p).await.unwrap()
}

pub async fn sou(Json(m): Json<Role>) -> R<Value> {
    role::sou(m).await.unwrap();
    R::ok()
}

pub async fn del(Json(ids): Json<Vec<u64>>) -> R<Value> {
    role::del(ids).await.unwrap();
    R::ok()
}

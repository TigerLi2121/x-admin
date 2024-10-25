use crate::common::res::R;
use crate::model::menu;
use crate::model::menu::Menu;
use axum::routing::get;
use axum::{Json, Router};
use serde_json::Value;

pub fn router() -> Router {
    Router::new().route("/", get(list).post(sou).delete(del))
}

pub async fn list() -> R<Vec<Menu>> {
    let ms = menu::list().await.unwrap_or(vec![]);
    let pm = ms.iter().filter(|m| m.pid == Some(0)).cloned().collect();
    let ms = build_menus(&pm, &ms);
    R::ok_data(ms)
}

pub async fn sou(Json(m): Json<Menu>) -> R<Value> {
    match menu::sou(m).await {
        Ok(_) => R::ok(),
        Err(e) => R::err_msg(e.to_string()),
    }
}

pub async fn del(Json(ids): Json<Vec<u64>>) -> R<Value> {
    match menu::del(ids).await {
        Ok(_) => R::ok(),
        Err(e) => R::err_msg(e.to_string()),
    }
}

pub fn build_menus(pms: &Vec<Menu>, ms: &Vec<Menu>) -> Vec<Menu> {
    pms.iter()
        .map(|pm| {
            let mut pm = pm.clone();
            let cms = ms.iter().filter(|m| m.pid == pm.id).cloned().collect();
            pm.children = Some(build_menus(&cms, &ms));
            pm
        })
        .collect()
}

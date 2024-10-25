use crate::common::jwt::get_token;
use crate::common::req::Page;
use crate::common::res::{R, RP};
use crate::model::menu::Menu;
use crate::model::user::{get_user, User};
use crate::model::{menu, user};
use axum::extract::{Query, Request};
use axum::routing::get;
use axum::{Json, Router};
use md5::{Digest, Md5};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::Error;
use std::fmt::Debug;

pub fn router() -> Router {
    Router::new()
        .route("/", get(page).post(sou).delete(del))
        .route("/current", get(current))
}

pub async fn login(Json(login): Json<Login>) -> R<String> {
    let user: Result<User, Error> = get_user(login.username.to_string()).await;
    if user.is_err() {
        return R::err_msg("username not exist".to_string());
    }
    let user = user.unwrap();
    if user.status != Some(1) {
        return R::err_msg("account deactivated".to_string());
    }
    let pwd = format!("{:x}", Md5::digest(login.password.as_bytes()));
    if user.password.unwrap() != pwd {
        return R::err_msg("password error".to_string());
    }
    let token = get_token(user.id.unwrap());
    R::ok_data(token)
}

pub async fn page(Query(p): Query<Page>) -> RP<Vec<User>> {
    user::page(p).await.unwrap()
}

pub async fn sou(Json(mut m): Json<User>) -> R<Value> {
    if m.password.is_some() && !m.password.clone().unwrap().is_empty() {
        m.password = Some(format!(
            "{:x}",
            Md5::digest(m.password.clone().unwrap().as_bytes())
        ));
    }
    user::sou(m).await.unwrap();
    R::ok()
}

pub async fn del(Json(ids): Json<Vec<u64>>) -> R<Value> {
    user::del(ids).await.unwrap();
    R::ok()
}

pub async fn current(req: Request) -> R<UserInfo> {
    let user = req.extensions().get::<User>().unwrap();
    let mut ms = menu::list_user_id(user.id.unwrap()).await.unwrap();
    let perms = ms
        .iter()
        .filter(|m| m.perms != Some("".to_string()))
        .map(|m| m.clone().perms.unwrap())
        .collect();
    ms = ms.into_iter().filter(|m| m.r#type != Some(3)).collect();
    let pms = ms
        .iter()
        .filter(|m| m.pid == Some(0))
        .map(|e| convert(e.clone()))
        .collect();
    let ui = UserInfo {
        id: user.id,
        username: user.username.clone(),
        email: user.email.clone(),
        mobile: user.mobile.clone(),
        perms: Some(perms),
        menus: Some(build_user_menus(&pms, &ms)),
    };
    R::ok_data(ui)
}

pub fn build_user_menus(pms: &Vec<UserMenu>, ms: &Vec<Menu>) -> Vec<UserMenu> {
    pms.iter()
        .map(|pm| {
            let mut pm = pm.clone();
            let cms = ms
                .iter()
                .filter(|m| m.pid == pm.id)
                .map(|e| convert(e.clone()))
                .collect::<Vec<UserMenu>>();
            if cms.is_empty() {
                pm.redirect = Some(format!(
                    "{}{}",
                    pm.path.clone().unwrap(),
                    pm.path.clone().unwrap()
                ));
                pm.meta.as_mut().unwrap().single = Some(true);
                pm.children = Some(vec![UserMenu {
                    id: pm.id.clone(),
                    path: Some(pm.path.clone().unwrap().replace("/", "")),
                    name: pm.name.clone(),
                    component: pm.component.clone(),
                    redirect: None,
                    meta: pm.meta.clone(),
                    children: Some(Vec::new()),
                }]);
            } else {
                pm.redirect = Some(format!(
                    "{}/{}",
                    pm.path.clone().unwrap(),
                    pm.path.clone().unwrap()
                ));
                pm.children = Some(build_user_menus(&cms, &ms));
            }
            pm.component = Some("LAYOUT".to_string());
            pm
        })
        .collect()
}

fn convert(menu: Menu) -> UserMenu {
    UserMenu {
        id: menu.id,
        path: menu.path,
        name: menu.name.clone(),
        component: menu.component,
        redirect: None,
        meta: Some(MenuMeta {
            title: menu.name,
            icon: menu.icon,
            single: Some(false),
            hidden: Some(menu.status == Some(2)),
        }),
        children: Some(Vec::new()),
    }
}

#[derive(Deserialize, Serialize, Debug)]
pub struct UserInfo {
    pub id: Option<u64>,
    pub username: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub perms: Option<Vec<String>>,
    pub menus: Option<Vec<UserMenu>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct UserMenu {
    pub id: Option<u64>,
    pub path: Option<String>,
    pub name: Option<String>,
    pub component: Option<String>,
    pub redirect: Option<String>,
    pub meta: Option<MenuMeta>,
    pub children: Option<Vec<UserMenu>>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct MenuMeta {
    pub title: Option<String>,
    pub icon: Option<String>,
    // 菜单是否展示一级节点
    pub single: Option<bool>,
    // 菜单是否展示
    pub hidden: Option<bool>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Login {
    username: String,
    password: String,
}

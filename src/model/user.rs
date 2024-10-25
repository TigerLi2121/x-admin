use crate::common::date_format;
use crate::common::db::get_pool;
use crate::common::req::Page;
use crate::common::res::RP;
use crate::model::user_role;
use serde::{Deserialize, Serialize};
use sqlx::types::chrono::NaiveDateTime;
use sqlx::{Error, FromRow, MySql, QueryBuilder};
use tracing::info;

pub async fn get_user(username: String) -> Result<User, Error> {
    let m = sqlx::query_as::<MySql, User>("SELECT * FROM user WHERE username = ? LIMIT 1")
        .bind(username)
        .fetch_one(get_pool().unwrap())
        .await?;
    Ok(m)
}

pub async fn page(page: Page) -> Result<RP<Vec<User>>, Error> {
    let count: i32 = sqlx::query_scalar("SELECT count(1) FROM user")
        .fetch_one(get_pool().unwrap())
        .await?;
    let mut ms: Vec<User> = vec![];
    if count > 0 {
        ms = sqlx::query_as("SELECT * FROM user ORDER BY id DESC LIMIT ? OFFSET ?")
            .bind(page.limit.to_string())
            .bind(page.offset().to_string())
            .fetch_all(get_pool().unwrap())
            .await?;
        // 获取用户角色id
        let user_ids = ms.iter().filter_map(|e| e.id).collect();
        let urs = user_role::get_user_ids(user_ids).await?;
        for m in ms.iter_mut() {
            m.role_ids = Some(
                urs.iter()
                    .filter(|e| e.user_id == m.id)
                    .filter_map(|e| e.role_id)
                    .collect(),
            );
        }
    }

    Ok(RP::ok(count, ms))
}

pub async fn sou(user: User) -> Result<u64, Error> {
    let row;
    if user.id.is_none() {
        info!("insert");
        row = sqlx::query::<MySql>(
            "INSERT INTO user (username,password,email,mobile,status) VALUES (?,?,?,?,?)",
        )
        .bind(user.username)
        .bind(user.password)
        .bind(user.email)
        .bind(user.mobile)
        .bind(user.status)
        .execute(get_pool().unwrap())
        .await?;
        info!("{} rows inserted", row.rows_affected());
    } else {
        let mut sql: QueryBuilder<MySql> = QueryBuilder::new("UPDATE user SET username=");
        sql.push_bind(user.username);
        if let Some(password) = user.password.as_ref().filter(|p| !p.is_empty()) {
            sql.push(",password=").push_bind(password);
        }
        sql.push(",email=").push_bind(user.email);
        sql.push(",mobile=").push_bind(user.mobile);
        sql.push(",status=").push_bind(user.status);
        sql.push(" WHERE id=").push_bind(user.id);
        println!("sql:{}", sql.sql());
        row = sql.build().execute(get_pool().unwrap()).await?;
        info!("{} rows updated", row.rows_affected());
    }

    // 保存用户角色
    user_role::batch_save(user.id.unwrap(), user.role_ids.unwrap()).await?;

    Ok(row.last_insert_id())
}

pub async fn del(ids: Vec<u64>) -> Result<(), Error> {
    let mut sql: QueryBuilder<MySql> = QueryBuilder::new("DELETE FROM user WHERE id IN (");
    let mut ps = sql.separated(", ");
    for id in ids.iter() {
        ps.push_bind(id);
    }
    ps.push_unseparated(") ");
    let row = sql.build().execute(get_pool().unwrap()).await?;
    info!("{} rows deleted", row.rows_affected());
    user_role::del_user_ids(ids).await?;
    Ok(())
}

#[derive(Debug, FromRow, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: Option<u64>,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub mobile: Option<String>,
    pub status: Option<i32>,
    #[serde(skip_deserializing)]
    #[serde(with = "date_format")]
    pub created_at: Option<NaiveDateTime>,
    #[serde(skip_deserializing)]
    #[serde(with = "date_format")]
    pub updated_at: Option<NaiveDateTime>,
    #[sqlx(skip)]
    pub role_ids: Option<Vec<u64>>,
}

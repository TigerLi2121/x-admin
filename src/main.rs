use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use common::db::init_db_pool;
use common::log::init_log;
use mid::auth::auth;
use tower_http::cors::{Any, CorsLayer};
use tracing::info;

mod common;
mod mid;
mod model;
mod route;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    init_db_pool().await.expect("connection db error");
    init_log().await;

    let router = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/api/login", post(route::user::login))
        .nest(
            "/api",
            Router::new()
                .nest("/user", route::user::router())
                .nest("/role", route::role::router())
                .nest("/menu", route::menu::router())
                .route_layer(middleware::from_fn(auth)),
        )
        .layer(middleware::from_fn(mid::api_log::log))
        .layer(CorsLayer::new().allow_methods(Any).allow_origin(Any));
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, router).await.unwrap();
}

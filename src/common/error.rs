use axum::{response::IntoResponse, BoxError};
use reqwest::StatusCode;

pub async fn global_error_handler(error: BoxError) -> impl IntoResponse {
    if error.is::<tower::timeout::error::Elapsed>() {
        (StatusCode::REQUEST_TIMEOUT, "请求超时")
    } else {
        (StatusCode::INTERNAL_SERVER_ERROR, "内部服务器错误")
    }
}
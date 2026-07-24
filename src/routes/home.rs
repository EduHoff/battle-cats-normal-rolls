use axum::{http::StatusCode, response::Html};

pub async fn home_page() -> Result<Html<String>, StatusCode> {
    tokio::fs::read_to_string("templates/index.html")
        .await
        .map(Html)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

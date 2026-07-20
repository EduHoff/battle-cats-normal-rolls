use axum::{Router, http::StatusCode, response::Html, routing::get};
use std::error::Error;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/", get(home_page))
        .nest_service("/static", ServeDir::new("static"));

    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Server running on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

async fn home_page() -> Result<Html<String>, StatusCode> {
    tokio::fs::read_to_string("templates/index.html")
        .await
        .map(Html)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)
}

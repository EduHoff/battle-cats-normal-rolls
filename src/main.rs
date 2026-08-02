use axum::{
    Router,
    routing::{get, post},
};
use battle_cats_normal_rolls::routes::{
    finder::{find_seed_handler, finder_page},
    home::home_page,
};
use std::{env, error::Error, sync::Arc};
use tera::Tera;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let mut tera = Tera::new();

    tera.add_template_files(vec![
        ("templates/index.html", Some("index.html")),
        ("templates/finder.html", Some("finder.html")),
        ("templates/header.html", Some("header.html")),
        ("templates/footer.html", Some("footer.html")),
    ])?;

    let app_state = Arc::new(tera);

    let app = Router::new()
        .route("/", get(home_page))
        .route("/finder", get(finder_page))
        .route("/find", post(find_seed_handler))
        .with_state(app_state)
        .nest_service("/static", ServeDir::new("static"));

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let addr = format!("{host}:{port}");

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    println!("Server running on http://{addr}");

    axum::serve(listener, app).await?;

    Ok(())
}

use axum::{
    Router,
    routing::{get /*post*/},
};
use battle_cats_normal_rolls::routes::{
    //finder::{find_seed_handler, finder_page},
    home::home_page,
};
use std::error::Error;
use tower_http::services::ServeDir;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let app = Router::new()
        .route("/", get(home_page))
        //.route("/finder", get(finder_page))
        //.route("/find", post(find_seed_handler))
        .nest_service("/static", ServeDir::new("static"));

    let addr = "127.0.0.1:3000";
    let listener = tokio::net::TcpListener::bind(addr).await?;
    println!("Server running on http://{}", addr);

    axum::serve(listener, app).await?;

    Ok(())
}

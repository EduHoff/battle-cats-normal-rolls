use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Html,
};
use serde::Deserialize;
use std::sync::Arc;
use tera::{Context, Tera};

use crate::domain::banner::BannerData;

#[derive(Deserialize)]
pub struct TrackParams {
    pub seed: Option<u32>,
    pub count: Option<usize>,
    pub event: Option<String>,
}

pub async fn home_page(
    State(tera): State<Arc<Tera>>,
    Query(params): Query<TrackParams>,
) -> Result<Html<String>, StatusCode> {
    let current_seed = params.seed.unwrap_or(1);
    let current_count = params.count.unwrap_or(100);

    let all_banners = BannerData::all_banners();
    let current_event = params.event.unwrap_or_else(|| {
        all_banners
            .first()
            .map(|b| b.short_name.clone())
            .unwrap_or_default()
    });

    let mut context = Context::new();
    context.insert("seed", &current_seed);
    context.insert("count", &current_count);
    context.insert("selected_event", &current_event);
    context.insert("banners", &all_banners);

    let empty_rows: Vec<String> = Vec::new(); // TEMPORARY FIX
    context.insert("rows", &empty_rows); // TEMPORARY FIX

    let rendered = tera.render("index.html", &context).map_err(|err| {
        eprintln!("Failed to render index.html template: {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Html(rendered))
}

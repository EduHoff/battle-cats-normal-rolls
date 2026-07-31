use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Html,
};
use serde::Deserialize;
use std::sync::Arc;
use tera::{Context, Tera};

use crate::domain::{banner::BannerData, seed::build_tracker_rows};

#[derive(Deserialize)]
pub struct TrackParams {
    pub seed: Option<u32>,
    pub count: Option<usize>,
    pub event: Option<String>,
    pub find: Option<String>,
}

pub async fn home_page(
    State(tera): State<Arc<Tera>>,
    Query(params): Query<TrackParams>,
) -> Result<Html<String>, StatusCode> {
    let current_seed = params.seed.unwrap_or(1);
    let current_count = params.count.unwrap_or(100);
    let selected_find = params.find.unwrap_or_default();

    let all_banners = BannerData::all_banners();
    let current_event = params.event.unwrap_or_else(|| {
        all_banners
            .first()
            .map(|b| b.short_name.clone())
            .unwrap_or_default()
    });

    let selected_banner = all_banners
        .iter()
        .find(|b| b.short_name == current_event)
        .unwrap_or(&all_banners[0]);

    let mut available_units: Vec<String> = selected_banner
            .pools
            .iter()
            .flat_map(|pool| pool.units.clone())
            .collect();
        available_units.sort();
        available_units.dedup();

    let rows = build_tracker_rows(current_seed, current_count, selected_banner);

    let mut context = Context::new();
    context.insert("seed", &current_seed);
    context.insert("count", &current_count);
    context.insert("selected_event", &selected_banner.short_name);
    context.insert("banners", &all_banners);
    context.insert("rows", &rows);
    context.insert("available_units", &available_units);
    context.insert("selected_find", &selected_find);

    let rendered = tera.render("index.html", &context).map_err(|err| {
        eprintln!("Failed to render index.html template: {err}");
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok(Html(rendered))
}

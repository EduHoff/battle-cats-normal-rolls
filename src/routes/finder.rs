use axum::{
    Form,
    extract::State,
    response::{Html, IntoResponse},
};
use std::sync::Arc;
use tera::{Context, Tera};

use crate::domain::banner::BannerData;
use crate::domain::finder::find_seeds;

pub async fn finder_page(State(tera): State<Arc<Tera>>) -> impl IntoResponse {
    let banners = BannerData::all_banners();

    let default_banner = banners
        .first()
        .expect("at least one banner must be available");

    let available_units = default_banner.all_units();

    let mut context = Context::new();
    context.insert("active_tab", "finder");
    context.insert("banners", &banners);
    context.insert("selected_event", &default_banner.short_name);
    context.insert("available_units", &available_units);
    context.insert("selected_rolls", &Vec::<String>::new());

    match tera.render("finder.html", &context) {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Template error: {err}"),
        )
            .into_response(),
    }
}

pub async fn find_seed_handler(
    State(tera): State<Arc<Tera>>,
    Form(raw_form): Form<Vec<(String, String)>>,
) -> impl IntoResponse {
    let mut selected_event = None;
    let mut rolls = Vec::new();

    for (key, value) in raw_form {
        if key == "event" {
            selected_event = Some(value);
        } else if key == "rolls" && !value.trim().is_empty() {
            rolls.push(value);
        }
    }

    let banners = BannerData::all_banners();
    let default_banner = banners.first().expect("banners list cannot be empty");

    let event_name = selected_event.unwrap_or_else(|| default_banner.short_name.clone());

    let banner = banners
        .iter()
        .find(|b| b.short_name == event_name)
        .unwrap_or(default_banner);

    let available_units = banner.all_units();

    let mut context = Context::new();
    context.insert("active_tab", "finder");
    context.insert("banners", &banners);
    context.insert("selected_event", &banner.short_name);
    context.insert("available_units", &available_units);
    context.insert("selected_rolls", &rolls);

    if rolls.len() < 3 {
        if !rolls.is_empty() {
            context.insert(
                "error",
                "Please select at least 3 rolls to perform a search.",
            );
        }
    } else {
        let results = find_seeds(&rolls, banner);
        if results.is_empty() {
            context.insert("error", "No seed matching the provided sequence was found.");
        } else {
            context.insert("results", &results);
        }
    }

    match tera.render("finder.html", &context) {
        Ok(html) => Html(html).into_response(),
        Err(err) => (
            axum::http::StatusCode::INTERNAL_SERVER_ERROR,
            format!("Template error: {err}"),
        )
            .into_response(),
    }
}

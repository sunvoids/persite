use askama::Template;
use axum::{
    http::{HeaderMap, StatusCode},
    response::{Html, IntoResponse},
};

use crate::routers::layout::LayoutTemplate;

pub async fn get(headers: HeaderMap) -> Result<impl IntoResponse, (StatusCode, String)> {
    let is_htmx = headers.contains_key("HX-Request");

    let index = IndexTemplate
        .render()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if is_htmx {
        Ok(Html(index))
    } else {
        let full = LayoutTemplate {
            title: "Index",
            content: index,
            categories: &[],
        }
        .render()
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

        Ok(Html(full))
    }
}

#[derive(Template)]
#[template(path = "index.html")]
struct IndexTemplate;

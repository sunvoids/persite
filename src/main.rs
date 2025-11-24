use axum::{
    Router, routing::get
};
use tower_http::{services::ServeDir, trace::{self, TraceLayer}};

mod routers;
use crate::routers::*;

mod db;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    let static_files = ServeDir::new("static");
    // build our application with a single route
    let app = Router::new()
        .layer(
            TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new())
            .on_response(trace::DefaultOnResponse::new())
        )
        .route("/", get(index::get))
        .route("/about", get(about::get))

        .nest_service("/static", static_files);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
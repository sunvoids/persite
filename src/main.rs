use axum::{
    Router, routing::get
};
use tower_http::{services::ServeDir, trace::{self, TraceLayer}};

mod routers;
use crate::routers::*;

mod db;
mod misc;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    //let env = misc::env_reader::get_env(".env").expect("[ERROR] Failed at reading .env file. Exiting.");
    
    let static_files = ServeDir::new("static");
    let app = Router::new()
        .layer(
            TraceLayer::new_for_http()
            .make_span_with(trace::DefaultMakeSpan::new())
            .on_response(trace::DefaultOnResponse::new())
        )
        .route("/", get(index::get))
        .route("/about", get(about::get))

        .nest_service("/static", static_files);
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}


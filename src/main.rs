use std::{error::Error, time::Duration};

use axum::{Router, routing::get};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectOptions, Database};
use tower_http::{
    services::ServeDir,
    trace::{self, TraceLayer},
};

mod routers;
use crate::routers::*;
mod db;
mod misc;

/// TODO: all unwraps here should be removed and handled
#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    // MOVE to db::connection
    let db_info =
        misc::env_reader::read_env(".env").expect("[ERROR] Failed at reading .env file. Exiting.");
    let db_url = format!(
        "postgres://{}:{}@{}/mydb",
        db_info.username, db_info.password, db_info.host
    );
    let mut db_connection_options = ConnectOptions::new(db_url);
    db_connection_options
        .max_connections(25)
        .min_connections(2)
        .connect_timeout(Duration::from_secs(8))
        .acquire_timeout(Duration::from_secs(8))
        .max_lifetime(Duration::from_hours(1))
        .sqlx_logging(true); // TODO: change to false on release build?
    let db = Database::connect(db_connection_options).await.unwrap();
    assert!(db.ping().await.is_ok());
    Migrator::up(&db, None).await.unwrap(); // This creates tables "categories" and "articles" if they do not exist. You're welcome to comment or remove if these tables exist.
    // ENDMOVE to db::connection
    let static_files = ServeDir::new("static");
    let app = Router::new()
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new())
                .on_response(trace::DefaultOnResponse::new()),
        )
        .route("/", get(index::get))
        .route("/about", get(about::get))
        .nest_service("/static", static_files);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
    
    db.close().await.unwrap();
    Ok(())
}

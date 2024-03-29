#![allow(unused)] // Temporariy

use std::net::SocketAddr;

use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
pub use error::{Error, Result};
use log::{debug, error, info, warn};
use serde::Deserialize;
use tokio::net::TcpListener;
use tower_http::cors::CorsLayer;

use crate::model::ModelController;

mod error;
mod helpers;
mod model;
mod routes_control;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let mc = ModelController::new().await?;

    let router = Router::new()
        .merge(routes_control::routes(mc.clone()))
        .layer(CorsLayer::permissive());

    // Start Server
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    info!("{:<12} - {:?}\n", "LISTENDING", listener.local_addr());
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

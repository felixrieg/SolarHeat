#![allow(unused)] // Temporary

use std::{net::SocketAddr, time::Duration};

use axum::{
    extract::Query,
    response::{Html, IntoResponse},
    routing::get,
    Router,
};
pub use error::{Error, Result};
use log::{debug, error, info, warn};
use serde::Deserialize;
use tokio::{
    net::TcpListener,
    spawn, task,
    time::{self, interval},
};
use tower_http::cors::CorsLayer;

//use crate::model::ModelController;

// use control::mod;
// use crate::control::model::ModelController;
mod model;
use model::model::ModelController;

mod error;
mod helpers;
mod web;
use web::routes;

use crate::model::model::StatusResponse;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let model = ModelController::new().await?;

    let router = routes::routes(model.clone()).layer(CorsLayer::permissive());

    // Start Server
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    info!("{:<12} - {:?}\n", "LISTENDING", listener.local_addr());
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

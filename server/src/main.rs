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

//use crate::model::ModelController;

// use control::mod;
// use crate::control::model::ModelController;
mod control;
use control::{control_model::ControlModelController, control_routes_control};

mod error;
mod helpers;
mod settings;
use settings::{settings_model::SettingsModelController, settings_routes_control};

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let mc = ControlModelController::new().await?;
    let settings_controller = SettingsModelController::new().await?;

    let router = Router::new()
        .nest(
            "/settings",
            settings_routes_control::routes(settings_controller.clone()),
        )
        .nest("/controls", control_routes_control::routes(mc.clone()))
        .layer(CorsLayer::permissive());

    // Start Server
    let listener = TcpListener::bind("0.0.0.0:8080").await.unwrap();
    info!("{:<12} - {:?}\n", "LISTENDING", listener.local_addr());
    axum::serve(listener, router.into_make_service())
        .await
        .unwrap();

    Ok(())
}

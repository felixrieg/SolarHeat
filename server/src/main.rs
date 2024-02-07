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

use crate::model::ModelController;

mod error;
mod helpers;
mod model;
mod routes_control;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();

    let mc = ModelController::new().await?;

    let routes_hello = Router::new()
        .route("/hello", get(handler_hello))
        .merge(routes_control::routes(mc.clone()));

    // Start Server
    let listener = TcpListener::bind("127.0.0.1:8080").await.unwrap();
    info!("{:<12} - {:?}\n", "LISTENDING", listener.local_addr());
    axum::serve(listener, routes_hello.into_make_service())
        .await
        .unwrap();

    // handlers

    #[derive(Debug, Deserialize)]
    struct HelloParams {
        name: Option<String>,
    }
    // e.g. /hello?name=Felix
    async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse {
        info!("{:<12} - handler_hello - {params:?}\n", "HANDLER");
        let name = params.name.as_deref().unwrap_or("NONAME");
        Html(format!(
            "Hello to the infamouse <strong>{name}</strong> place"
        ))
    }

    Ok(())
}

use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use log::info;
use serde::{Deserialize, Serialize};

use crate::{
    model::{ControlData, ModelController, Modus},
    Result,
};

#[derive(Debug, Clone, Deserialize)]
pub struct ChangeModusData {
    pub modus: Modus,
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangePosData {
    pub lat: f32,
    pub lon: f32,
}

pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/modus", post(change_modus))
        .route("/controls", get(get_state))
        .route("/position", post(change_pos))
        .with_state(mc)
}

async fn change_modus(
    State(mc): State<ModelController>,
    Json(modus): Json<ChangeModusData>,
) -> Result<Json<ControlData>> {
    info!("{:<12} - {:?}\n", "change_modus", "HANDLER");

    let new_control_data = mc.change_modus(modus.modus).await?;

    Ok(Json(new_control_data))
}

async fn get_state(State(mc): State<ModelController>) -> Result<Json<ControlData>> {
    // info!("{:<12} - {:?}\n", "get_state", "HANDLER");

    let control_data = mc.get_data().await?;

    info!(
        "{:<12} - {:?} : {:?}\n",
        "get_state", "HANDLER", control_data
    );

    Ok(Json(control_data))
}

async fn change_pos(
    State(mc): State<ModelController>,
    Json(new_pos): Json<ChangePosData>,
) -> Result<Json<ControlData>> {
    info!("{:<12} - {:?}\n", "change_pos", "HANDLER");

    let new_control_data = mc.change_pos(new_pos.lat, new_pos.lon).await?;

    Ok(Json(new_control_data))
}

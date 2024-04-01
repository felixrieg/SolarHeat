use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use log::info;
use serde::{Deserialize, Serialize};

use crate::{
    control::control_model::{ControlData, ControlModelController, Modus},
    Result,
};

pub fn routes(mc: ControlModelController) -> Router {
    Router::new()
        .route("/", get(get_state))
        .route("/modus", post(change_modus))
        .route("/weather", post(change_weather))
        .with_state(mc)
}

async fn get_state(State(mc): State<ControlModelController>) -> Result<Json<ControlData>> {
    // info!("{:<12} - {:?}\n", "get_state", "HANDLER");

    let control_data = mc.get_data().await?;

    info!(
        "{:<12} - {:?} : {:?}\n",
        "get_state", "HANDLER", control_data
    );

    Ok(Json(control_data))
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangeModusData {
    pub modus: Modus,
}

async fn change_modus(
    State(mc): State<ControlModelController>,
    Json(modus): Json<ChangeModusData>,
) -> Result<Json<ControlData>> {
    info!("{:<12} - {:?}\n", "change_modus", "HANDLER");

    let new_control_data = mc.change_modus(modus.modus).await?;

    Ok(Json(new_control_data))
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangeWeatherData {
    pub weather: f32,
}

async fn change_weather(
    State(mc): State<ControlModelController>,
    Json(data): Json<ChangeWeatherData>,
) -> Result<Json<ControlData>> {
    info!("{:<12} - {:?}\n", "change_modus", "HANDLER");

    let new_control_data = mc.change_weather(data.weather).await?;

    Ok(Json(new_control_data))
}

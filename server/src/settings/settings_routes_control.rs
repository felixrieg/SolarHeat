use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use log::info;
use serde::{Deserialize, Serialize};

use crate::{
    settings::settings_model::{SettingsData, SettingsModelController},
    Result,
};

// pub struct SettingsData {
//     pub lat: f32,
//     pub lon: f32,
//     pub house_rotation: f32,
//     pub roof_inclination: f32,
//     pub start_value: f32,
//     pub end_value: f32,
// }

pub fn routes(mc: SettingsModelController) -> Router {
    Router::new()
        .route("/", get(get_state))
        .route("/position", post(change_pos))
        .route("/rotation", post(change_house_rotation))
        .route("/incline", post(change_roof_inclination))
        .route("/start", post(change_start_value))
        .route("/end", post(change_end_value))
        .with_state(mc)
}

async fn get_state(State(mc): State<SettingsModelController>) -> Result<Json<SettingsData>> {
    // info!("{:<12} - {:?}\n", "get_state", "HANDLER");

    let control_data = mc.get_data().await?;

    info!(
        "{:<12} - {:?} : {:?}\n",
        "get_state", "HANDLER", control_data
    );

    Ok(Json(control_data))
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangePosData {
    pub lat: f32,
    pub lon: f32,
}

async fn change_pos(
    State(mc): State<SettingsModelController>,
    Json(new_pos): Json<ChangePosData>,
) -> Result<Json<SettingsData>> {
    info!("{:<12} - {:?}\n", "change_pos", "HANDLER");

    let new_control_data = mc.change_pos(new_pos.lat, new_pos.lon).await?;

    Ok(Json(new_control_data))
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangeHouseRotation {
    pub rotation: f32,
}

async fn change_house_rotation(
    State(mc): State<SettingsModelController>,
    Json(new_pos): Json<ChangeHouseRotation>,
) -> Result<Json<SettingsData>> {
    info!("{:<12} - {:?}\n", "change_pos", "HANDLER");

    let new_control_data = mc.change_house_rotation(new_pos.rotation).await?;

    Ok(Json(new_control_data))
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangeRoofInclination {
    pub inclination: f32,
}

async fn change_roof_inclination(
    State(mc): State<SettingsModelController>,
    Json(new_pos): Json<ChangeRoofInclination>,
) -> Result<Json<SettingsData>> {
    info!("{:<12} - {:?}\n", "change_pos", "HANDLER");

    let new_control_data = mc.change_roof_inclination(new_pos.inclination).await?;

    Ok(Json(new_control_data))
}

#[derive(Debug, Clone, Deserialize)]
pub struct ChangeValue {
    pub value: f32,
}

async fn change_start_value(
    State(mc): State<SettingsModelController>,
    Json(new_pos): Json<ChangeValue>,
) -> Result<Json<SettingsData>> {
    info!("{:<12} - {:?}\n", "change_pos", "HANDLER");

    let new_control_data = mc.change_start_value(new_pos.value).await?;

    Ok(Json(new_control_data))
}

async fn change_end_value(
    State(mc): State<SettingsModelController>,
    Json(new_pos): Json<ChangeValue>,
) -> Result<Json<SettingsData>> {
    info!("{:<12} - {:?}\n", "change_pos", "HANDLER");

    let new_control_data = mc.change_end_value(new_pos.value).await?;

    Ok(Json(new_control_data))
}

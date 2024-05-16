use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use log::info;

use crate::{
    helpers::is_rising,
    model::{control_model::ControlData, model::ModelController, settings_model::SettingsData},
};
use crate::{
    helpers::{get_altitude, get_altitude_in},
    model::{control_model::Modus, model::StatusResponse},
    Error, Result,
};
// Import the missing `Handler` trait
pub fn routes(mc: ModelController) -> Router {
    Router::new()
        .route("/ping", get(ping))
        .route("/settings", get(get_settings))
        .route("/settings", post(set_settings))
        .route("/controls", get(get_controls))
        // Implement the `Handler` trait for `set_controls`
        .route("/controls", post(set_controls))
        .route("/status", get(get_status))
        .with_state(mc)
}

async fn ping() -> Result<String> {
    Ok(String::from("Hello, World!"))
}

async fn get_settings(State(mc): State<ModelController>) -> Result<Json<SettingsData>> {
    let settings_data = mc.get_settings_data().await?;

    info!(
        "{:<12} - {:?} : {:?}\n",
        "get_settings", "HANDLER", settings_data
    );

    Ok(Json(settings_data))
}

async fn set_settings(
    State(mc): State<ModelController>,
    Json(new_state): Json<SettingsData>,
) -> Result<Json<SettingsData>> {
    info!(
        "{:<12} - {:?}: {:?}\n",
        "set_settings", "HANDLER", new_state
    );

    let new_settings_data = mc.set_settings_data(new_state).await?;

    Ok(Json(new_settings_data))
}

async fn get_controls(State(mc): State<ModelController>) -> Result<Json<ControlData>> {
    let control_data = mc.get_control_data().await?;

    info!(
        "{:<12} - {:?} : {:?}\n",
        "get_controls", "HANDLER", control_data
    );

    Ok(Json(control_data))
}

async fn set_controls(
    State(mc): State<ModelController>,
    Json(new_state): Json<ControlData>,
) -> Result<Json<ControlData>> {
    info!(
        "{:<12} - {:?}: {:?}\n",
        "set_controls", "HANDLER", new_state
    );

    let new_control_data = mc.set_control_data(new_state).await?;

    Ok(Json(new_control_data))
}

async fn get_status(State(mc): State<ModelController>) -> Result<Json<StatusResponse>> {
    let control_data = mc.get_control_data().await?;
    let settings_data = mc.get_settings_data().await?;

    info!(
        "{:<12} - {:?} : {:?}\n",
        "get_status", "HANDLER", control_data
    );

    let altitude = get_altitude(settings_data.lat as f64, settings_data.lon as f64);

    let status = match control_data.modus {
        Modus::On => true,
        Modus::Off => false,
        Modus::Continuous => {
            if is_rising(settings_data.lat as f64, settings_data.lon as f64) {
                info!("{:<12} - {:?}", "sun", "rising");
                altitude > settings_data.start_value as f64
            } else {
                info!("{:<12} - {:?}", "sun", "setting");
                altitude > settings_data.end_value as f64
            }
        }
        Modus::Single => {
            if is_rising(settings_data.lat as f64, settings_data.lon as f64) {
                info!("{:<12} - {:?}", "sun", "rising");
                altitude > settings_data.start_value as f64
            } else {
                info!("{:<12} - {:?}", "sun", "setting");
                let now = altitude > settings_data.end_value as f64;
                let later =
                    (get_altitude_in(settings_data.lat as f64, settings_data.lon as f64, |x| {
                        x + 300000
                    }) > settings_data.end_value as f64); // 5 minutes later
                if now && !later {
                    // When switching status to off while modus is single, the modus is set to off
                    mc.set_control_data(ControlData {
                        modus: Modus::Off,
                        weather: control_data.weather,
                    })
                    .await?;
                }
                now
            }
        }
    };

    info!("{:<12} - {:?}", "sun", altitude);
    Ok(Json(StatusResponse {
        status: status,
        pin: settings_data.pin,
    }))
}

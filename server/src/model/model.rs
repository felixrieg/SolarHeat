use serde::{Deserialize, Serialize};

use crate::{helpers::get_altitude, Error, Result};
use std::sync::{Arc, Mutex};

use super::{control_model::ControlData, settings_model::SettingsData};

#[derive(Clone)]
pub struct ModelController {
    pub control: Arc<Mutex<ControlData>>,
    pub settings: Arc<Mutex<SettingsData>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            control: Arc::default(),
            settings: Arc::default(),
        })
    }
}

impl ModelController {
    pub async fn get_control_data(&self) -> Result<ControlData> {
        let store = self.control.lock().unwrap();
        Ok(store.clone())
    }

    pub async fn set_control_data(&self, new_state: ControlData) -> Result<ControlData> {
        let mut store = self.control.lock().unwrap();
        store.modus = new_state.modus;
        store.weather = new_state.weather;
        Ok(store.clone())
    }

    pub async fn get_settings_data(&self) -> Result<SettingsData> {
        let store = self.settings.lock().unwrap();
        Ok(store.clone())
    }

    pub async fn set_settings_data(&self, new_state: SettingsData) -> Result<SettingsData> {
        let mut store = self.settings.lock().unwrap();
        store.lat = new_state.lat;
        store.lon = new_state.lon;
        store.house_rotation = new_state.house_rotation;
        store.roof_inclination = new_state.roof_inclination;
        store.start_value = new_state.start_value;
        store.end_value = new_state.end_value;
        store.pin = new_state.pin;
        Ok(store.clone())
    }
}

// Other Models

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub status: bool,
    pub pin: u32,
}

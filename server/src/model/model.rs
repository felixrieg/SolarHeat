use serde::{Deserialize, Serialize};

use crate::{constants::STORAGE_FILE, helpers::get_altitude, Error, Result};
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
    pub async fn to_string(&self) -> Result<String> {
        let control = self.control.lock().unwrap();
        let settings = self.settings.lock().unwrap();

        let model_store = ModelStore {
            control: control.clone(),
            settings: settings.clone(),
        };

        Ok(serde_json::to_string(&model_store)?)
    }

    pub async fn from_string(data: String) -> Result<Self> {
        let model_store: ModelStore = serde_json::from_str(&data)?;

        Ok(Self {
            control: Arc::new(Mutex::new(model_store.control)),
            settings: Arc::new(Mutex::new(model_store.settings)),
        })
    }

    pub async fn save_default(&self) -> Result<()> {
        self.save(STORAGE_FILE).await
    }

    pub async fn save(&self, file: &str) -> Result<()> {
        let data = self.to_string().await?;
        tokio::fs::write(file, data).await?;
        Ok(())
    }

    pub async fn load(file: &str) -> Result<Self> {
        let data = tokio::fs::read_to_string(file).await?;
        Self::from_string(data).await
    }

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
        store.default_high = new_state.default_high;
        Ok(store.clone())
    }
}

impl Default for ModelController {
    fn default() -> Self {
        Self {
            control: Arc::default(),
            settings: Arc::default(),
        }
    }
}

// Other Models for IO or requests

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StatusResponse {
    pub status: bool,
    pub pin: u32,
    pub pin_state: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ModelStore {
    pub control: ControlData,
    pub settings: SettingsData,
}

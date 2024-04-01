use std::{
    default,
    sync::{Arc, Mutex},
    time::{Duration, Instant, SystemTime},
};

use crate::{helpers::get_altitude, Error, Result};
use log::info;
use serde::{Deserialize, Serialize};
use sun;

#[derive(Debug, Clone, Serialize)]
pub struct SettingsData {
    pub lat: f32,
    pub lon: f32,
    pub house_rotation: f32,
    pub roof_inclination: f32,
    pub start_value: f32,
    pub end_value: f32,
}

impl Default for SettingsData {
    fn default() -> Self {
        Self {
            lat: 0.0,
            lon: 0.0,
            house_rotation: 0.0,
            roof_inclination: 0.0,
            start_value: 0.0,
            end_value: 0.0,
        }
    }
}

// ModelControler
#[derive(Clone)]
pub struct SettingsModelController {
    control_data_store: Arc<Mutex<SettingsData>>,
}

impl SettingsModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            control_data_store: Arc::default(),
        })
    }
}

// CRUD Implementation

impl SettingsModelController {
    pub async fn get_data(&self) -> Result<SettingsData> {
        let store = self.control_data_store.lock().unwrap();
        Ok(store.clone())
    }

    pub async fn change_pos(&self, lat: f32, lon: f32) -> Result<SettingsData> {
        let mut store = self.control_data_store.lock().unwrap();
        store.lat = lat;
        store.lon = lon;
        Ok(store.clone())
    }

    pub async fn change_house_rotation(&self, rot: f32) -> Result<SettingsData> {
        let mut store = self.control_data_store.lock().unwrap();
        store.house_rotation = rot;
        Ok(store.clone())
    }

    pub async fn change_roof_inclination(&self, incl: f32) -> Result<SettingsData> {
        let mut store = self.control_data_store.lock().unwrap();
        store.roof_inclination = incl;
        Ok(store.clone())
    }

    pub async fn change_start_value(&self, start: f32) -> Result<SettingsData> {
        let mut store = self.control_data_store.lock().unwrap();
        store.start_value = start;
        Ok(store.clone())
    }

    pub async fn change_end_value(&self, end: f32) -> Result<SettingsData> {
        let mut store = self.control_data_store.lock().unwrap();
        store.end_value = end;
        Ok(store.clone())
    }
}

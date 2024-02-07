use std::{
    default,
    sync::{Arc, Mutex},
    time::{Duration, Instant, SystemTime},
};

use crate::{helpers::get_altitude, Error, Result};
use log::info;
use serde::{Deserialize, Serialize};
use sun;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Modus {
    Off,
    Single,
    Continuous,
}

#[derive(Debug, Clone, Serialize)]
pub struct ControlData {
    pub modus: Modus,
    pub lat: f32,
    pub lon: f32,
    pub status: bool,
}

impl Default for ControlData {
    fn default() -> Self {
        Self {
            modus: Modus::Off,
            lat: 0.0,
            lon: 0.0,
            status: false,
        }
    }
}

// ModelControler

#[derive(Clone)]
pub struct ModelController {
    control_data_store: Arc<Mutex<ControlData>>,
}

impl ModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            control_data_store: Arc::default(),
        })
    }
}

// CRUD Implementation

impl ModelController {
    pub async fn get_data(&self) -> Result<ControlData> {
        let store = self.control_data_store.lock().unwrap();
        Ok(store.clone())
    }

    pub async fn change_modus(&self, modus: Modus) -> Result<ControlData> {
        let mut store = self.control_data_store.lock().unwrap();
        store.modus = modus.clone();
        store.status = Self::process_status(&store.clone());
        Ok(store.clone())
    }

    pub async fn change_pos(&self, lat: f32, lon: f32) -> Result<ControlData> {
        let mut store = self.control_data_store.lock().unwrap();
        store.lat = lat;
        store.lon = lon;
        store.status = Self::process_status(&store.clone());
        Ok(store.clone())
    }

    pub fn process_status(state: &ControlData) -> bool {
        let altitude = get_altitude(state.lat as f64, state.lon as f64);

        info!("{:<12} - {:?}", "sun", altitude);

        match state.modus {
            Modus::Continuous | Modus::Single => {
                if altitude > 10.0 {
                    true
                } else {
                    false
                }
            }
            _ => false,
        }
    }
}

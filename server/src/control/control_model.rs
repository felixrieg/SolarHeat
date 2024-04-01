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
    On,
}

#[derive(Debug, Clone, Serialize)]
pub struct ControlData {
    pub modus: Modus,
    pub weather: f32,
}

impl Default for ControlData {
    fn default() -> Self {
        Self {
            modus: Modus::Off,
            weather: 0.5,
        }
    }
}

// ModelControler

#[derive(Clone)]
pub struct ControlModelController {
    control_data_store: Arc<Mutex<ControlData>>,
}

impl ControlModelController {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            control_data_store: Arc::default(),
        })
    }
}

// CRUD Implementation

impl ControlModelController {
    pub async fn get_data(&self) -> Result<ControlData> {
        let store = self.control_data_store.lock().unwrap();
        Ok(store.clone())
    }

    pub async fn change_modus(&self, modus: Modus) -> Result<ControlData> {
        let mut store = self.control_data_store.lock().unwrap();
        store.modus = modus.clone();
        Ok(store.clone())
    }

    pub async fn change_weather(&self, weather: f32) -> Result<ControlData> {
        let mut store = self.control_data_store.lock().unwrap();
        store.weather = weather;
        Ok(store.clone())
    }

    // pub fn process_status(state: &ControlData) -> bool {
    //     let altitude = get_altitude(state.lat as f64, state.lon as f64);

    //     info!("{:<12} - {:?}", "sun", altitude);

    //     match state.modus {
    //         Modus::Continuous | Modus::Single => {
    //             if altitude > 10.0 {
    //                 true
    //             } else {
    //                 false
    //             }
    //         }
    //         _ => false,
    //     }
    // }
}

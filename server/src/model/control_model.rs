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

#[derive(Debug, Clone, Serialize, Deserialize)]
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

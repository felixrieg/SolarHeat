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
pub struct SettingsData {
    pub lat: f32,
    pub lon: f32,
    pub house_rotation: f32,
    pub roof_inclination: f32,
    pub start_value: f32,
    pub end_value: f32,
    pub pin: u32,
}

impl Default for SettingsData {
    fn default() -> Self {
        Self {
            lat: 48.39953,
            lon: 9.98331,
            house_rotation: 0.0,
            roof_inclination: 0.0,
            start_value: 5.0,
            end_value: 5.0,
            pin: 25,
        }
    }
}

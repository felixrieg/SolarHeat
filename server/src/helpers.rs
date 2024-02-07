use std::time::{Duration, SystemTime, UNIX_EPOCH};

use log::info;

pub fn get_altitude(lat: f64, lon: f64) -> f64 {
    get_altitude_at(
        lat,
        lon,
        SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap_or(Duration::from_secs(69))
            .as_millis() as i64,
    )
}

pub fn get_altitude_at(lat: f64, lon: f64, timestamp: i64) -> f64 {
    let unixtime = suncalc::Timestamp(timestamp);
    let pos = suncalc::get_position(unixtime, lat, lon);
    pos.altitude.to_degrees()
}

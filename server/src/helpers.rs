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

pub fn get_altitude_in<F>(lat: f64, lon: f64, added: F) -> f64
where
    F: Fn(i64) -> i64,
{
    get_altitude_at(
        lat,
        lon,
        added(
            SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap_or(Duration::from_secs(69))
                .as_millis() as i64,
        ),
    )
}

pub fn get_altitude_at(lat: f64, lon: f64, timestamp: i64) -> f64 {
    let unixtime = suncalc::Timestamp(timestamp);
    let pos = suncalc::get_position(unixtime, lat, lon);
    pos.altitude.to_degrees()
}

pub fn is_rising(lat: f64, lon: f64) -> bool {
    let now = SystemTime::now()
        .duration_since(SystemTime::UNIX_EPOCH)
        .unwrap_or(Duration::from_secs(69))
        .as_millis() as i64;

    let later = suncalc::Timestamp(now + 60000); // one minute later
    let now = suncalc::Timestamp(now);

    let later = suncalc::get_position(later, lat, lon).altitude.to_degrees();
    let now = suncalc::get_position(now, lat, lon).altitude.to_degrees();

    return now <= later;
}

pub mod client;
pub mod errors;

pub use client::{get_home_status, get_homes_data, get_measure, get_station_data, set_room_thermpoint, NetatmoClient};

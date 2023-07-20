use crate::{client::NetatmoClient, errors::Result};

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use super::get_homes_data::GatewayType;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HomeStatus {
    pub status: String,
    pub time_server: i64,
    pub body: HomeStatusBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HomeStatusBody {
    pub home: Home,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Home {
    pub id: String,
    pub modules: Vec<Module>,
    pub rooms: Vec<Room>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Module {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub firmware_revision: i64,
    pub rf_strength: Option<i64>,
    pub wifi_strength: Option<i64>,
    pub reachable: Option<bool>,
    pub battery_level: Option<i64>,
    pub boiler_valve_comfort_boost: Option<bool>,
    pub boiler_status: Option<bool>,
    pub anticipating: Option<bool>,
    pub bridge: Option<String>,
    pub battery_state: Option<String>,
    pub status_active: Option<bool>,
    pub status_tampered: Option<bool>,
    pub test_mode: Option<bool>,
    pub hush_mode: Option<bool>,
    pub smoke_detected: Option<bool>,
    pub detection_chamber_status: Option<String>,
    pub battery_alarm_state: Option<String>,
    pub battery_percent: Option<i64>,
    pub wifi_status: Option<i64>,
    pub last_smoke_detected_start_time: Option<i64>,
    pub last_smoke_detected_end_time: Option<i64>,
    pub last_seen: Option<i64>,
    pub last_wifi_connection: Option<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub reachable: bool,
    pub therm_measured_temperature: f64,
    pub heating_power_request: i64,
    pub therm_setpoint_temperature: f64,
    pub therm_setpoint_mode: String,
    pub therm_setpoint_start_time: i64,
    pub therm_setpoint_end_time: i64,
    pub anticipating: bool,
    pub open_window: bool,
}

#[derive(Default)]
pub struct GetHomeStatusParameters {
    home_id: Option<String>,
    device_types: Option<Vec<GatewayType>>,
}

impl GetHomeStatusParameters {
    pub fn new() -> Self {
        GetHomeStatusParameters::default()
    }

    pub fn home_id(self, home_id: &str) -> Self {
        GetHomeStatusParameters {
            home_id: Some(home_id.to_string()),
            ..self
        }
    }

    pub fn device_types(self, device_types: &[GatewayType]) -> Self {
        GetHomeStatusParameters {
            device_types: Some(device_types.to_vec()),
            ..self
        }
    }
}

#[allow(clippy::implicit_hasher)]
impl From<&GetHomeStatusParameters> for HashMap<String, String> {
    fn from(p: &GetHomeStatusParameters) -> HashMap<String, String> {
        let mut map = HashMap::default();
        if let Some(home_id) = &p.home_id {
            map.insert("home_id".to_string(), home_id.to_string());
        }
        if let Some(device_types) = &p.device_types {
            let device_types = device_types
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .as_slice()
                .join(",");
            map.insert("device_types".to_string(), device_types);
        }

        map
    }
}

pub async fn get_home_status(client: &NetatmoClient, parameters: &GetHomeStatusParameters) -> Result<HomeStatus> {
    let params: HashMap<String, String> = parameters.into();
    let mut params = params.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    client
        .call("get_home_status", "https://api.netatmo.com/api/homestatus", &mut params)
        .await
}

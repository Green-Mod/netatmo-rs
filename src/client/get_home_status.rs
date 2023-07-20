use super::get_homes_data::GatewayType;
use crate::{
    client::NetatmoClient,
    errors::{NetatmoError, Result},
};
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::{collections::HashMap, fmt, str::FromStr};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HomeStatus {
    pub status: String,
    pub time_server: i64,
    pub body: HomeStatusBody,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HomeStatusBody {
    pub home: Home,
    pub errors: Option<Vec<HomeStatusError>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Home {
    pub id: String,
    pub modules: Option<Vec<Module>>,
    pub rooms: Option<Vec<Room>>,
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
    pub heating_power_request: i64,
    pub therm_measured_temperature: f64,
    pub therm_setpoint_temperature: f64,
    pub therm_setpoint_mode: ThermSetpointMode,
    pub therm_setpoint_start_time: i64,
    pub therm_setpoint_end_time: i64,
    pub anticipating: bool,
    pub open_window: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThermSetpointMode {
    #[default]
    Manual,
    Max,
    Off,
    Schedule,
    Away,
    Hg,
}

impl fmt::Display for ThermSetpointMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            ThermSetpointMode::Manual => "manual",
            ThermSetpointMode::Max => "max",
            ThermSetpointMode::Off => "off",
            ThermSetpointMode::Schedule => "schedule",
            ThermSetpointMode::Away => "away",
            ThermSetpointMode::Hg => "hg",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for ThermSetpointMode {
    type Err = NetatmoError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "manual" => Ok(ThermSetpointMode::Manual),
            "max" => Ok(ThermSetpointMode::Max),
            "off" => Ok(ThermSetpointMode::Off),
            "schedule" => Ok(ThermSetpointMode::Schedule),
            "away" => Ok(ThermSetpointMode::Away),
            "hg" => Ok(ThermSetpointMode::Hg),
            _ => Err(NetatmoError::FailedToReadResponse),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HomeStatusError {
    pub code: HomeStatusErrorCode,
    pub id: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum HomeStatusErrorCode {
    #[default]
    UnknownError = 1,
    InternalError = 2,
    ParserError = 3,
    CommandUnknownNodeModuleError = 4,
    CommandInvalidParams = 5,
    Unreachable = 6,
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

#[cfg(test)]
mod test {
    use super::*;

    mod get_home_status {
        use super::*;

        #[test]
        fn parse_response() {
            let json = r#"{
                "status": "ok",
                "time_server": 1689865621,
                "body": {
                  "home": {
                    "id": "609f88d35d96af39490be591",
                    "modules": [
                      {
                        "id": "70:ee:50:7d:6b:fa",
                        "type": "NSD",
                        "firmware_revision": 108,
                        "last_seen": 1622622024,
                        "wifi_strength": 35
                      }
                    ]
                  },
                  "errors": [
                    {
                      "code": 6,
                      "id": "70:ee:50:72:99:6a"
                    }
                  ]
                }
              }"#;

            let station_data: std::result::Result<HomeStatus, _> = serde_json::from_str(json);

            assert!(&station_data.is_ok());
        }
    }
}

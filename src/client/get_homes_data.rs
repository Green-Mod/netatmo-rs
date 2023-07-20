use crate::{client::NetatmoClient, errors::Result};

use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HomesData {
    pub body: HomesDataBody,
    pub status: String,
    pub time_exec: f64,
    pub time_server: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HomesDataBody {
    pub homes: Vec<Home>,
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Home {
    pub id: String,
    pub name: String,
    pub timezone: String,
    pub rooms: Vec<Room>,
    pub modules: Vec<Module>,
    pub therm_schedules: Vec<ThermSchedule>,
    pub therm_setpoint_default_duration: i64,
    pub therm_mode: String,
    pub schedules: Vec<Schedule>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub module_ids: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Module {
    pub id: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub name: String,
    pub setup_date: i64,
    pub modules_bridged: Option<Vec<String>>,
    pub room_id: Option<String>,
    pub bridge: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ThermSchedule {
    pub timetable: Vec<Timetable>,
    pub zones: Vec<Zone>,
    pub name: String,
    pub default: bool,
    pub away_temp: i64,
    pub hg_temp: i64,
    pub id: String,
    pub selected: bool,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Timetable {
    pub zone_id: i64,
    pub m_offset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Zone {
    pub name: String,
    pub id: i64,
    #[serde(rename = "type")]
    pub type_field: i64,
    pub rooms_temp: Vec<RoomsTemp>,
    pub rooms: Option<Vec<RoomTemp>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomsTemp {
    pub room_id: String,
    pub temp: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    pub timetable: Vec<Timetable>,
    pub zones: Vec<Zone>,
    pub name: String,
    pub default: bool,
    pub away_temp: i64,
    pub hg_temp: i64,
    pub id: String,
    pub selected: bool,
    #[serde(rename = "type")]
    pub type_field: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RoomTemp {
    pub id: String,
    pub therm_setpoint_temperature: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct User {
    pub email: String,
    pub language: String,
    pub locale: String,
    pub feel_like_algorithm: i64,
    pub unit_pressure: i64,
    pub unit_system: i64,
    pub unit_wind: i64,
    pub id: String,
}

#[derive(Default)]
pub struct GetHomesDataParameters {
    home_id: Option<String>,
    gateway_types: Option<Vec<GatewayType>>,
}

impl GetHomesDataParameters {
    pub fn new() -> Self {
        GetHomesDataParameters::default()
    }

    pub fn home_id(self, home_id: &str) -> Self {
        GetHomesDataParameters {
            home_id: Some(home_id.to_string()),
            ..self
        }
    }

    pub fn gateway_types(self, gateway_types: &[GatewayType]) -> Self {
        GetHomesDataParameters {
            gateway_types: Some(gateway_types.to_vec()),
            ..self
        }
    }
}

#[derive(Debug, Clone)]
pub enum GatewayType {
    ThermostatValve,
    Welcome,
    Presence,
}

impl fmt::Display for GatewayType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            GatewayType::ThermostatValve => "NAPLUG",
            GatewayType::Welcome => "Humidity",
            GatewayType::Presence => "NOC",
        };
        write!(f, "{}", s)
    }
}

#[allow(clippy::implicit_hasher)]
impl From<&GetHomesDataParameters> for HashMap<String, String> {
    fn from(p: &GetHomesDataParameters) -> HashMap<String, String> {
        let mut map = HashMap::default();
        if let Some(home_id) = &p.home_id {
            map.insert("home_id".to_string(), home_id.to_string());
        }
        if let Some(gateway_types) = &p.gateway_types {
            let gateway_types = gateway_types
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>()
                .as_slice()
                .join(",");
            map.insert("gateway_types".to_string(), gateway_types);
        }

        map
    }
}

pub async fn get_homes_data(client: &NetatmoClient, parameters: &GetHomesDataParameters) -> Result<HomesData> {
    let params: HashMap<String, String> = parameters.into();
    let mut params = params.iter().map(|(k, v)| (k.clone(), v.clone())).collect();
    client
        .call("get_homes_data", "https://api.netatmo.com/api/homesdata", &mut params)
        .await
}

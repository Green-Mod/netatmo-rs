use crate::{
    client::NetatmoClient,
    errors::{NetatmoError, Result},
};
use serde::{Deserialize, Serialize};
use serde_repr::*;
use std::{collections::HashMap, fmt, str::FromStr};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HomesData {
    pub body: HomesDataBody,
    pub status: String,
    pub time_exec: f64,
    pub time_server: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct HomesDataBody {
    pub homes: Option<Vec<Home>>,
    pub user: User,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Home {
    pub id: String,
    pub name: String,
    pub timezone: String,
    pub rooms: Option<Vec<Room>>,
    pub modules: Option<Vec<Module>>,
    pub therm_setpoint_default_duration: Option<i64>,
    pub therm_mode: Option<ThermMode>,
    pub schedules: Option<Vec<Schedule>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ThermMode {
    #[default]
    Schedule,
    Away,
    FrostGuard,
}

impl fmt::Display for ThermMode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            ThermMode::Schedule => "schedule",
            ThermMode::Away => "away",
            ThermMode::FrostGuard => "hg",
        };
        write!(f, "{}", s)
    }
}

impl FromStr for ThermMode {
    type Err = NetatmoError;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "schedule" => Ok(ThermMode::Schedule),
            "away" => Ok(ThermMode::Away),
            "hg" => Ok(ThermMode::FrostGuard),
            _ => Err(NetatmoError::FailedToReadResponse),
        }
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Room {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub type_field: String,
    pub module_ids: Option<Vec<String>>,
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
pub struct Timetable {
    pub zone_id: i64,
    pub m_offset: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Zone {
    pub name: String,
    pub id: i64,
    #[serde(rename = "type")]
    pub type_field: ZoneType,
    pub rooms: Option<Vec<RoomTemp>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum ZoneType {
    #[default]
    Day = 0,
    Night = 1,
    Away = 2,
    FrostGuard = 3,
    Custom = 4,
    Eco = 5,
    Comfort = 8,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Schedule {
    pub timetable: Option<Vec<Timetable>>,
    pub zones: Option<Vec<Zone>>,
    pub name: String,
    pub default: bool,
    pub away_temp: i64,
    pub hg_temp: i64,
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
    NAPlug,
    OTH,
    BNS,
}

impl fmt::Display for GatewayType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            GatewayType::NAPlug => "NAPlug",
            GatewayType::OTH => "OTH",
            GatewayType::BNS => "BNS",
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

#[cfg(test)]
mod test {
    use super::*;

    mod get_homes_data {
        use super::*;

        #[test]
        fn parse_response() {
            let json = r#"{
                "body": {
                  "homes": [
                    {
                      "id": "...",
                      "name": "Home",
                      "altitude": 50,
                      "coordinates": [
                        82.5057837,
                        -62.5575262
                      ],
                      "country": "CAN",
                      "timezone": "EDT",
                      "rooms": [
                        {
                          "id": "...",
                          "name": "...",
                          "type": "bedroom"
                        }
                      ],
                      "schedules": [
                        {
                          "timetable": [
                            {
                              "zone_id": 1,
                              "m_offset": 0
                            },
                            {
                              "zone_id": 0,
                              "m_offset": 480
                            },
                            {
                              "zone_id": 4,
                              "m_offset": 525
                            },
                            {
                              "zone_id": 0,
                              "m_offset": 1140
                            },
                            {
                              "zone_id": 1,
                              "m_offset": 1380
                            },
                            {
                              "zone_id": 0,
                              "m_offset": 1920
                            },
                            {
                              "zone_id": 4,
                              "m_offset": 1965
                            },
                            {
                              "zone_id": 0,
                              "m_offset": 2580
                            },
                            {
                              "zone_id": 1,
                              "m_offset": 2820
                            },
                            {
                              "zone_id": 0,
                              "m_offset": 3360
                            },
                            {
                              "zone_id": 4,
                              "m_offset": 3405
                            },
                            {
                              "zone_id": 0,
                              "m_offset": 4020
                            },
                            {
                              "zone_id": 1,
                              "m_offset": 4260
                            },
                            {
                              "zone_id": 0,
                              "m_offset": 4800
                            },
                            {
                              "zone_id": 4,
                              "m_offset": 4845
                            },
                            {
                              "zone_id": 0,
                              "m_offset": 5460
                            },
                            {
                              "zone_id": 1,
                              "m_offset": 5700
                            },
                            {
                              "zone_id": 0,
                              "m_offset": 6240
                            },
                            {
                              "zone_id": 4,
                              "m_offset": 6285
                            },
                            {
                              "zone_id": 0,
                              "m_offset": 6900
                            },
                            {
                              "zone_id": 1,
                              "m_offset": 7140
                            },
                            {
                              "zone_id": 0,
                              "m_offset": 7740
                            },
                            {
                              "zone_id": 1,
                              "m_offset": 8625
                            },
                            {
                              "zone_id": 0,
                              "m_offset": 9180
                            },
                            {
                              "zone_id": 1,
                              "m_offset": 10065
                            }
                          ],
                          "zones": [
                            {
                              "name": "Comfort",
                              "id": 0,
                              "type": 0,
                              "rooms_temp": [
                                {
                                  "room_id": "...",
                                  "temp": 17
                                }
                              ],
                              "rooms": [
                                {
                                  "id": "...",
                                  "therm_setpoint_temperature": 17
                                }
                              ]
                            },
                            {
                              "name": "Night",
                              "id": 1,
                              "type": 1,
                              "rooms_temp": [
                                {
                                  "room_id": "...",
                                  "temp": 17
                                }
                              ],
                              "rooms": [
                                {
                                  "id": "...",
                                  "therm_setpoint_temperature": 17
                                }
                              ]
                            },
                            {
                              "name": "Comfort+",
                              "id": 3,
                              "type": 8,
                              "rooms_temp": [
                                {
                                  "room_id": "...",
                                  "temp": 17
                                }
                              ],
                              "rooms": [
                                {
                                  "id": "...",
                                  "therm_setpoint_temperature": 17
                                }
                              ]
                            },
                            {
                              "name": "Eco",
                              "id": 4,
                              "type": 5,
                              "rooms_temp": [
                                {
                                  "room_id": "...",
                                  "temp": 16
                                }
                              ],
                              "rooms": [
                                {
                                  "id": "...",
                                  "therm_setpoint_temperature": 16
                                }
                              ]
                            }
                          ],
                          "name": "...",
                          "default": false,
                          "away_temp": 12,
                          "hg_temp": 7,
                          "id": "...",
                          "selected": true,
                          "type": "therm"
                        }
                      ]
                    }
                  ],
                  "user": {
                    "email": "giorgio@greenmod.it",
                    "language": "it-IT",
                    "locale": "it-IT",
                    "feel_like_algorithm": 0,
                    "unit_pressure": 0,
                    "unit_system": 0,
                    "unit_wind": 0,
                    "id": "..."
                  }
                },
                "status": "ok",
                "time_exec": 0.020753145217895508,
                "time_server": 1689864276
              }"#;

            let homes_data: std::result::Result<HomesData, _> = serde_json::from_str(json);

            assert!(&homes_data.is_ok());
        }
    }
}

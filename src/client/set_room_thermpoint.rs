use crate::{client::NetatmoClient, errors::Result};
use serde::Deserialize;
use std::{collections::HashMap, fmt};

pub struct SetRoomThermpointParameters {
    home_id: String,
    room_id: String,
    mode: Mode,
    temp: Option<f32>,
    endtime: Option<usize>,
}

pub enum Mode {
    Manual,
    Home,
}

impl fmt::Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Mode::Manual => "manual",
            Mode::Home => "home",
        };
        write!(f, "{}", s)
    }
}

impl SetRoomThermpointParameters {
    pub fn new(home_id: &str, room_id: &str, mode: Mode) -> Self {
        SetRoomThermpointParameters {
            home_id: home_id.to_string(),
            room_id: room_id.to_string(),
            mode,
            temp: None,
            endtime: None,
        }
    }

    pub fn temp(self, temp: f32) -> Self {
        SetRoomThermpointParameters {
            temp: Some(temp),
            ..self
        }
    }

    pub fn date_end(self, date_end: usize) -> Self {
        SetRoomThermpointParameters {
            endtime: Some(date_end),
            ..self
        }
    }
}

#[allow(clippy::implicit_hasher)]
impl From<&SetRoomThermpointParameters> for HashMap<String, String> {
    fn from(p: &SetRoomThermpointParameters) -> HashMap<String, String> {
        let mut map = HashMap::default();
        map.insert("home_id".to_string(), p.home_id.to_string());
        map.insert("room_id".to_string(), p.room_id.to_string());
        map.insert("mode".to_string(), p.mode.to_string());
        if let Some(temp) = p.temp {
            map.insert("temp".to_string(), temp.to_string());
        }
        if let Some(endtime) = p.endtime {
            map.insert("endtime".to_string(), endtime.to_string());
        }

        map
    }
}

#[derive(Deserialize, Debug)]
pub struct SetRoomThermpointResponse {
    pub status: String,
    pub time_server: usize,
}

// cf. https://dev.netatmo.com/resources/technical/reference/energy/setroomthermpoint
pub async fn set_room_thermpoint(
    client: &NetatmoClient,
    parameters: &SetRoomThermpointParameters,
) -> Result<SetRoomThermpointResponse> {
    let params: HashMap<String, String> = parameters.into();
    let mut params = params.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

    client
        .call(
            "set_room_thermpoint",
            "https://api.netatmo.com/api/setroomthermpoint",
            &mut params,
        )
        .await
}

use crate::{client::AuthenticatedClient, errors::Result};

use serde::Deserialize;
use std::{collections::HashMap, fmt};

pub struct Parameters {
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

impl Parameters {
    pub fn new(home_id: &str, room_id: &str, mode: Mode) -> Self {
        Parameters {
            home_id: home_id.to_string(),
            room_id: room_id.to_string(),
            mode,
            temp: None,
            endtime: None,
        }
    }

    pub fn temp(self, temp: f32) -> Self {
        Parameters {
            temp: Some(temp),
            ..self
        }
    }

    pub fn date_end(self, date_end: usize) -> Self {
        Parameters {
            endtime: Some(date_end),
            ..self
        }
    }
}

pub enum Type {
    Temperature,
    Humidity,
    CO2,
}

impl fmt::Display for Type {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Type::Temperature => "Temperature",
            Type::Humidity => "Humidity",
            Type::CO2 => "CO2",
        };
        write!(f, "{}", s)
    }
}

#[allow(clippy::implicit_hasher)]
impl From<&Parameters> for HashMap<String, String> {
    fn from(p: &Parameters) -> HashMap<String, String> {
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
pub struct Response {
    pub status: String,
    pub time_server: usize,
}

// cf. https://dev.netatmo.com/resources/technical/reference/energy/setroomthermpoint
pub async fn set_room_thermpoint(client: &AuthenticatedClient, parameters: &Parameters) -> Result<Response> {
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

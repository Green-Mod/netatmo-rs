use crate::{client::NetatmoClient, errors::Result};
use serde::{Deserialize, Deserializer, Serialize};
use std::{collections::HashMap, fmt, str::FromStr};

pub struct GetMeasureParameters {
    device_id: String,
    module_id: String,
    scale: Scale,
    types: Vec<Type>,
    date_begin: Option<usize>,
    date_end: Option<usize>,
    limit: Option<bool>,
    real_time: Option<bool>,
}

impl GetMeasureParameters {
    pub fn new(device_id: &str, scale: Scale, types: &[Type]) -> Self {
        GetMeasureParameters {
            device_id: device_id.to_string(),
            module_id: device_id.to_string(),
            scale,
            types: types.to_vec(),
            date_begin: None,
            date_end: None,
            limit: None,
            real_time: None,
        }
    }

    pub fn with_module_id(device_id: &str, module_id: &str, scale: Scale, types: &[Type]) -> Self {
        GetMeasureParameters {
            device_id: device_id.to_string(),
            module_id: module_id.to_string(),
            scale,
            types: types.to_vec(),
            date_begin: None,
            date_end: None,
            limit: None,
            real_time: None,
        }
    }

    pub fn date_begin(self, date_begin: usize) -> Self {
        GetMeasureParameters {
            date_begin: Some(date_begin),
            ..self
        }
    }

    pub fn date_end(self, date_end: usize) -> Self {
        GetMeasureParameters {
            date_end: Some(date_end),
            ..self
        }
    }

    pub fn limit(self, limit: bool) -> Self {
        GetMeasureParameters {
            limit: Some(limit),
            ..self
        }
    }

    pub fn real_time(self, real_time: bool) -> Self {
        GetMeasureParameters {
            real_time: Some(real_time),
            ..self
        }
    }
}

pub enum Scale {
    Max,
    Min30,
    Hour1,
    Hours3,
    Day1,
    Week1,
    Month1,
}

impl fmt::Display for Scale {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match self {
            Scale::Max => "max",
            Scale::Min30 => "30min",
            Scale::Hour1 => "1hour",
            Scale::Hours3 => "3hours",
            Scale::Day1 => "1day",
            Scale::Week1 => "1week",
            Scale::Month1 => "1month",
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug, Clone)]
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
impl From<&GetMeasureParameters> for HashMap<String, String> {
    fn from(p: &GetMeasureParameters) -> HashMap<String, String> {
        let types = p
            .types
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .as_slice()
            .join(",");
        let mut m = HashMap::default();
        m.insert("device_id".to_string(), p.device_id.to_string());
        m.insert("module_id".to_string(), p.module_id.to_string());
        m.insert("scale".to_string(), p.scale.to_string());
        m.insert("type".to_string(), types);
        if let Some(date_begin) = p.date_begin {
            m.insert("date_begin".to_string(), date_begin.to_string());
        }
        if let Some(date_end) = p.date_end {
            m.insert("date_end".to_string(), date_end.to_string());
        }
        if let Some(limit) = p.limit {
            m.insert("limit".to_string(), limit.to_string());
        }
        m.insert("optimize".to_string(), "false".to_string());
        if let Some(real_time) = p.real_time {
            m.insert("real_time".to_string(), real_time.to_string());
        }

        m
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Measure {
    status: String,
    time_exec: f64,
    #[serde(rename = "body", deserialize_with = "de_body_values")]
    values: HashMap<usize, Vec<Option<f64>>>,
}

// cf. https://dev.netatmo.com/resources/technical/reference/common/getmeasure
pub async fn get_measure(client: &NetatmoClient, parameters: &GetMeasureParameters) -> Result<Measure> {
    let params: HashMap<String, String> = parameters.into();
    let mut params = params.iter().map(|(k, v)| (k.clone(), v.clone())).collect();

    client
        .call("get_measure", "https://api.netatmo.com/api/getmeasure", &mut params)
        .await
}

fn de_body_values<'de, D>(deserializer: D) -> ::std::result::Result<HashMap<usize, Vec<Option<f64>>>, D::Error>
where
    D: Deserializer<'de>,
{
    let map = HashMap::<String, Vec<Option<f64>>>::deserialize(deserializer)?;
    let mut tuples = Vec::new();
    for (k, v) in map {
        let key = usize::from_str(&k).map_err(serde::de::Error::custom)?;
        tuples.push((key, v));
    }
    let res = tuples.into_iter().collect();

    Ok(res)
}

#[cfg(test)]
mod test {
    use super::*;

    mod get_measure {
        use super::*;

        #[test]
        fn parse_response() {
            let json = r#"{
                "body": {
                  "1623794400": [
                    1429,
                    1000
                  ],
                  "1626386400": [
                    653
                  ]
                },
                "status": "ok",
                "time_exec": 0.039312124252319336,
                "time_server": 1689866240
              }"#;

            let measure: std::result::Result<Measure, _> = serde_json::from_str(json);

            assert!(&measure.is_ok());
        }
    }
}

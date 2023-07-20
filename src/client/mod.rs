use crate::errors::{NetatmoError, Result};
use get_home_status::HomeStatus;
use get_homes_data::HomesData;
use get_measure::Measure;
use get_station_data::StationData;
use log::trace;
use reqwest::{Client, Response, StatusCode};
use serde::de::DeserializeOwned;
use serde::Deserialize;
use std::collections::HashMap;

use self::{
    get_home_status::{get_home_status, GetHomeStatusParameters},
    get_homes_data::{get_homes_data, GetHomesDataParameters},
    get_measure::{get_measure, GetMeasureParameters},
    get_station_data::{get_homecoachs_data, get_station_data},
    set_room_thermpoint::{set_room_thermpoint, SetRoomThermpointParameters, SetRoomThermpointResponse},
};

pub mod get_home_status;
pub mod get_homes_data;
pub mod get_measure;
pub mod get_station_data;
pub mod set_room_thermpoint;

pub struct NetatmoClient {
    token: String,
    http: Client,
}

impl NetatmoClient {
    pub fn with_token(access_token: String) -> Self {
        Self {
            token: access_token,
            http: Client::new(),
        }
    }

    pub fn token(&self) -> &String {
        &self.token
    }

    pub async fn call<T>(&self, name: &str, url: &str, params: &mut HashMap<String, String>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        params.insert("access_token".to_string(), self.token.clone());
        api_call(name, &self.http, url, params).await
    }
}

async fn api_call<T>(name: &str, http: &Client, url: &str, params: &HashMap<String, String>) -> Result<T>
where
    T: DeserializeOwned,
{
    let res = http
        .post(url)
        .form(&params)
        .send()
        .await
        .map_err(|_| NetatmoError::FailedToSendRequest)?;

    let res = general_err_handler(res, name.to_string(), StatusCode::OK).await?;

    let status = res.status();
    let body = res.text().await.map_err(|_| NetatmoError::FailedToReadResponse)?;
    trace!("Sucessful ({:?}) repsone: '{}'", status, body);
    serde_json::from_str::<T>(&body).map_err(|_| NetatmoError::JsonDeserializationFailed)
}

#[derive(Debug, Deserialize)]
struct ApiError {
    #[serde(rename = "error")]
    details: ApiErrorDetails,
}

#[derive(Debug, Deserialize)]
struct ApiErrorDetails {
    code: isize,
    message: String,
}

async fn general_err_handler(response: Response, name: String, expected_status: StatusCode) -> Result<Response> {
    match response.status() {
        code if code == expected_status => Ok(response),
        code @ StatusCode::BAD_REQUEST
        | code @ StatusCode::UNAUTHORIZED
        | code @ StatusCode::FORBIDDEN
        | code @ StatusCode::NOT_FOUND
        | code @ StatusCode::NOT_ACCEPTABLE
        | code @ StatusCode::INTERNAL_SERVER_ERROR => {
            let body = response.text().await.map_err(|_| NetatmoError::UnknownApiCallFailure {
                name: name.clone(),
                status_code: code.as_u16(),
            })?;
            let err: ApiError = serde_json::from_str(&body).map_err(|_| NetatmoError::UnknownApiCallFailure {
                name: name.clone(),
                status_code: code.as_u16(),
            })?;
            Err(NetatmoError::ApiCallFailed {
                name,
                code: err.details.code,
                msg: err.details.message,
            })
        }
        code => Err(NetatmoError::UnknownApiCallFailure {
            name,
            status_code: code.as_u16(),
        }),
    }
}

impl NetatmoClient {
    pub async fn get_homes_data(&self, parameters: &GetHomesDataParameters) -> Result<HomesData> {
        get_homes_data(self, parameters).await
    }

    pub async fn get_home_status(&self, parameters: &GetHomeStatusParameters) -> Result<HomeStatus> {
        get_home_status(self, parameters).await
    }

    pub async fn get_station_data(&self, device_id: &str) -> Result<StationData> {
        get_station_data(self, device_id).await
    }

    pub async fn get_homecoachs_data(&self, device_id: &str) -> Result<StationData> {
        get_homecoachs_data(self, device_id).await
    }

    pub async fn get_measure(&self, parameters: &GetMeasureParameters) -> Result<Measure> {
        get_measure(self, parameters).await
    }

    pub async fn set_room_thermpoint(
        &self,
        parameters: &SetRoomThermpointParameters,
    ) -> Result<SetRoomThermpointResponse> {
        set_room_thermpoint(self, parameters).await
    }
}

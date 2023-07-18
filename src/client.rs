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

pub mod get_home_status;
pub mod get_homes_data;
pub mod get_measure;
pub mod get_station_data;
pub mod set_room_thermpoint;

pub struct NetatmoClient {}

impl NetatmoClient {
    pub fn with_token(access_token: String) -> AuthenticatedClient {
        AuthenticatedClient {
            token: access_token,
            http: Client::new(),
        }
    }
}

pub struct AuthenticatedClient {
    token: String,
    http: Client,
}

impl AuthenticatedClient {
    pub fn token(&self) -> &String {
        &self.token
    }

    pub async fn call<'a, T>(&self, name: &'static str, url: &str, params: &mut HashMap<String, String>) -> Result<T>
    where
        T: DeserializeOwned,
    {
        params.insert("access_token".to_string(), self.token.clone());
        api_call(name, &self.http, url, params).await
    }
}

async fn api_call<T>(name: &'static str, http: &Client, url: &str, params: &HashMap<String, String>) -> Result<T>
where
    T: DeserializeOwned,
{
    let res = http
        .post(url)
        .form(&params)
        .send()
        .await
        .map_err(|_| NetatmoError::FailedToSendRequest)?;

    let res = general_err_handler(res, name, StatusCode::OK).await?;

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

async fn general_err_handler(response: Response, name: &'static str, expected_status: StatusCode) -> Result<Response> {
    match response.status() {
        code if code == expected_status => Ok(response),
        code @ StatusCode::BAD_REQUEST
        | code @ StatusCode::UNAUTHORIZED
        | code @ StatusCode::FORBIDDEN
        | code @ StatusCode::NOT_FOUND
        | code @ StatusCode::NOT_ACCEPTABLE
        | code @ StatusCode::INTERNAL_SERVER_ERROR => {
            let body = response.text().await.map_err(|_| NetatmoError::UnknownApiCallFailure {
                name,
                status_code: code.as_u16(),
            })?;
            let err: ApiError = serde_json::from_str(&body).map_err(|_| NetatmoError::UnknownApiCallFailure {
                name,
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

impl AuthenticatedClient {
    pub async fn get_homes_data(&self, parameters: &get_homes_data::Parameters) -> Result<HomesData> {
        get_homes_data::get_homes_data(self, parameters).await
    }

    pub async fn get_home_status(&self, parameters: &get_home_status::Parameters) -> Result<HomeStatus> {
        get_home_status::get_home_status(self, parameters).await
    }

    pub async fn get_station_data(&self, device_id: &str) -> Result<StationData> {
        get_station_data::get_station_data(self, device_id).await
    }

    pub async fn get_homecoachs_data(&self, device_id: &str) -> Result<StationData> {
        get_station_data::get_homecoachs_data(self, device_id).await
    }

    pub async fn get_measure(&self, parameters: &get_measure::Parameters) -> Result<Measure> {
        get_measure::get_measure(self, parameters).await
    }

    pub async fn set_room_thermpoint(
        &self,
        parameters: &set_room_thermpoint::Parameters,
    ) -> Result<set_room_thermpoint::Response> {
        set_room_thermpoint::set_room_thermpoint(self, parameters).await
    }
}

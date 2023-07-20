use netatmo_rs::client::{
    get_homes_data::{GatewayType, GetHomesDataParameters},
    NetatmoClient,
};
use std::env;

#[tokio::main]
async fn main() {
    let access_token = env::var_os("NETATMO_ACCESS_TOKEN")
        .expect("Environment variable 'NETATMO_ACCESS_TOKEN' is not set.")
        .to_string_lossy()
        .to_string();
    let home_id = env::var_os("NETATMO_HOME_ID")
        .expect("Environment variable 'NETATMO_HOME_ID' is not set")
        .to_string_lossy()
        .to_string();

    let m_params = GetHomesDataParameters::new()
        .home_id(&home_id) // to fetch for only one home
        .gateway_types(&[GatewayType::ThermostatValve]); // to fetch for only a specific type of device

    let homes_data = NetatmoClient::with_token(access_token)
        .get_homes_data(&m_params)
        .await
        .expect("Failed to get homes data");

    println!("{:#?}", homes_data);
}

use netatmo_rs::client::{
    get_measure::{GetMeasureParameters, Scale, Type},
    NetatmoClient,
};
use std::env;

#[tokio::main]
async fn main() {
    let access_token = env::var_os("NETATMO_ACCESS_TOKEN")
        .expect("Environment variable 'NETATMO_ACCESS_TOKEN' is not set.")
        .to_string_lossy()
        .to_string();
    let device_id = env::var_os("NETATMO_DEVICE_ID")
        .expect("Environment variable 'NETATMO_DEVICE_ID' is not set")
        .to_string_lossy()
        .to_string();

    let m_params = GetMeasureParameters::new(&device_id, Scale::Max, &[Type::Humidity, Type::Temperature, Type::CO2]);

    let station_data = NetatmoClient::with_token(access_token)
        .get_measure(&m_params)
        .await
        .expect("Failed to get measure");

    println!("{:#?}", station_data);
}

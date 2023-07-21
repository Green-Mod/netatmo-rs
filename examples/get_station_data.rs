use netatmo_rs::client::NetatmoClient;
use std::env;

#[tokio::main]
async fn main() {
    env_logger::init();

    let access_token = env::var_os("NETATMO_ACCESS_TOKEN")
        .expect("Environment variable 'NETATMO_ACCESS_TOKEN' is not set.")
        .to_string_lossy()
        .to_string();
    let device_id = env::var_os("NETATMO_DEVICE_ID")
        .expect("Environment variable 'NETATMO_DEVICE_ID' is not set")
        .to_string_lossy()
        .to_string();

    let station_data = NetatmoClient::with_token(&access_token)
        .get_station_data(&device_id)
        .await
        .expect("Failed to get station data");

    println!("{:#?}", station_data);
}

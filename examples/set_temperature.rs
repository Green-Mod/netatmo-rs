use netatmo_rs::{
    set_room_thermpoint::{Mode, Parameters},
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
    let room_id = env::var_os("NETATMO_ROOM_ID")
        .expect("Environment variable 'NETATMO_ROOM_ID' is not set")
        .to_string_lossy()
        .to_string();

    let m_params = Parameters::new(&home_id, &room_id, Mode::Home);

    NetatmoClient::with_token(access_token)
        .set_room_thermpoint(&m_params)
        .await
        .expect("Failed to set home thermpoint");
}

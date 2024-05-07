//! main.rs
//! sonic main

use rspotify::{
    Config,
};

use log::debug;

// libs
mod auth;
mod logging;
// mod pkce;

#[tokio::main]
async fn main() {
    // log init
    let _handle = logging::log_init();

    // configure client
    let config: Config = Config {
        token_cached: true,
        token_refreshing: true,
        ..Default::default()
    };
    debug!("config: {:?}", config);

    let spotify = auth::auth_code_pkce_flow().await;

    // let artists = spotify
    //     .artist(ArtistId::from_id("0OdUWJ0sBjDrqHygGUXeCF").unwrap())
    //     .await;
    // debug!("artists: {:?}", artists);

    // let user = spotify
    //     .user(UserId::from_id("5sjbm8n5l7i0nlucndi0yp2cm").unwrap())
    //     .await
    //     .unwrap();
    // debug!("user: {:?}", user);
}
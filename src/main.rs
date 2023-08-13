use rspotify::{
    model::{AdditionalType, ArtistId, Country, Market, UserId},
    prelude::*,
    scopes, AuthCodeSpotify, ClientCredsSpotify, ClientError, ClientResult, Config, Credentials,
    OAuth,
};

use log::{debug, error, info};

// libs
mod auth;
mod logging;

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

    // create client
    // TODO: Don't use AuthCodeSpotify (Authorization code) authentication, since this method
    // is not suitable for applications, where client id and secret can't be safely stored.
    // Instead, implement AuthCodePkceSpotify (Authorization code with PKCE) authentication.
    // This method is also encourages by Spotify.
    let spotify = auth::auth_code_flow(config).await;

    let artists = spotify
        .artist(ArtistId::from_id("0OdUWJ0sBjDrqHygGUXeCF").unwrap())
        .await;
    debug!("artists: {:?}", artists);

    let user = spotify
        .user(UserId::from_id("5sjbm8n5l7i0nlucndi0yp2cm").unwrap())
        .await
        .unwrap();
    debug!("user: {:?}", user);
}

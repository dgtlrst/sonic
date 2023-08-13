use rspotify::{
    model::{AdditionalType, ArtistId, Country, Market, UserId},
    prelude::*,
    scopes, AuthCodeSpotify, ClientCredsSpotify, ClientError, ClientResult, Config, Credentials,
    OAuth,
};

use log::{debug, error, info};
use std::env;

// libs
mod logging;

#[tokio::main]
async fn main() {
    // log init
    let _handle = logging::log_init();

    // TODO: Don't use AuthCodeSpotify (Authorization code) authentication, since this method
    // is not suitable for applications, where client id and secret can't be safely stored.
    // Instead, implement AuthCodePkceSpotify (Authorization code with PKCE) authentication.
    // This method is also encourages by Spotify.
    env::set_var("RSPOTIFY_CLIENT_ID", "291fcee697f94999bf71fbf499bdfb54");
    env::set_var("RSPOTIFY_CLIENT_SECRET", "439e720432e149c0bb3176a3b5e1713c");
    env::set_var("RSPOTIFY_REDIRECT_URI", "https://example.org/callback");

    // hardcode some credentials (this should change to either get from env file or env vars, which are set separately)
    let creds = Credentials::from_env().unwrap();
    debug!("creds: {:?}", creds);

    let oauth = OAuth::from_env(scopes!("playlist-read-private")).unwrap();
    debug!("oauth: {:?}", oauth);

    // initialize spotify client
    let config: Config = Config {
        token_cached: true,
        token_refreshing: true,
        ..Default::default()
    };
    debug!("config: {:?}", config);
    let spotify = AuthCodeSpotify::with_config(creds, oauth, config);
    debug!("spotify: {:?}", spotify);

    // prompt for token
    let url = spotify.get_authorize_url(false).unwrap();

    match spotify.prompt_for_token(&url).await {
        rspotify::ClientResult::Ok(()) => println!("token: {:?}", spotify.get_token()),
        Err(e) => println!("error: {:?}", e),
    }

    let code = spotify.get_code_from_user(&url);
    debug!("code: {:?}", code);

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

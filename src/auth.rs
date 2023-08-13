// performs authentication with spotify

use log::{debug, error, info};
use std::env;

use tokio;

use rspotify::{
    model::{AdditionalType, ArtistId, Country, Market, UserId},
    prelude::*,
    scopes, AuthCodePkceSpotify, AuthCodeSpotify, ClientCredsSpotify, ClientError, ClientResult,
    Config, Credentials, OAuth,
};

const RSPOTIFY_CLIENT_ID: &str = "291fcee697f94999bf71fbf499bdfb54";
const RSPOTIFY_CLIENT_SECRET: &str = "439e720432e149c0bb3176a3b5e1713c";
const RSPOTIFY_REDIRECT_URI: &str = "https://example.org/callback";

fn _set_env_var() {
    env::set_var("RSPOTIFY_CLIENT_ID", RSPOTIFY_CLIENT_ID);
    env::set_var("RSPOTIFY_CLIENT_SECRET", RSPOTIFY_CLIENT_SECRET);
    env::set_var("RSPOTIFY_REDIRECT_URI", RSPOTIFY_REDIRECT_URI);
}

fn creds() -> Credentials {
    return Credentials::from_env().unwrap();
}

fn oauth() -> OAuth {
    // TODO: hardcoded scopes
    return OAuth::from_env(scopes!("playlist-read-private")).unwrap();
}

fn ac_spotify_client(creds: Credentials, oauth: OAuth, config: Config) -> AuthCodeSpotify {
    // AuthCodeSpotify is used for authorization code flow
    return AuthCodeSpotify::with_config(creds, oauth, config);
}

pub async fn auth_code_flow(config: Config) -> AuthCodeSpotify {
    _set_env_var();

    // init credentials
    let creds = creds();
    debug!("creds: {:?}", creds);

    // init oauth
    let oauth = oauth();
    debug!("oauth: {:?}", oauth);

    let spotify = ac_spotify_client(creds, oauth, config);
    debug!("spotify: {:?}", spotify);

    // prompt for token
    let url = spotify.get_authorize_url(false).unwrap();

    match spotify.prompt_for_token(&url).await {
        rspotify::ClientResult::Ok(()) => debug!("token: {:?}", spotify.get_token()),
        Err(e) => error!("error: {:?}", e),
    }

    let code = spotify.get_code_from_user(&url);
    debug!("code: {:?}", code);

    return spotify;
}

pub async fn auth_code_pkce_flow(config: Config) -> AuthCodePkceSpotify {
    unimplemented!();
}

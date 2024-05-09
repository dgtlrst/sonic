//! auth.rs
//! performs authentication with spotify

use std::{env};
use rspotify::{scopes, AuthCodePkceSpotify, Credentials, OAuth};

const RSPOTIFY_CLIENT_ID: &str = "291fcee697f94999bf71fbf499bdfb54";
const RSPOTIFY_CLIENT_SECRET: &str = "439e720432e149c0bb3176a3b5e1713c";
const RSPOTIFY_REDIRECT_URI: &str = "http://localhost:8888/callback";

fn _set_env_var() {
    env::set_var("RSPOTIFY_CLIENT_ID", RSPOTIFY_CLIENT_ID);
    env::set_var("RSPOTIFY_CLIENT_SECRET", RSPOTIFY_CLIENT_SECRET);
    env::set_var("RSPOTIFY_REDIRECT_URI", RSPOTIFY_REDIRECT_URI);
}

fn _creds() -> Credentials {
    return Credentials::from_env().unwrap();
}

fn _oauth() -> OAuth {
    // TODO: un-hardcode scopes, they are used to choose what kind of access we are requesting
    return OAuth::from_env(scopes!("playlist-read-private")).unwrap();
}

pub async fn auth_code_pkce_flow() -> AuthCodePkceSpotify {
    _set_env_var();                     // set the environment variables

    let creds = _creds();
    let oauth = _oauth();

    let mut spotify = AuthCodePkceSpotify::new(creds.clone(), oauth.clone());

    let url = spotify.get_authorize_url(None).unwrap();

    // perform an http request to the url
    let response = reqwest::get(&url).await.unwrap();

    let urlresp = response.url().to_string();
    println!("Response: {urlresp}");
    match webbrowser::open(&url) {
        Ok(_) => log::debug!("[AUTH] Opened {} in your browser.", url),
        Err(why) => log::error!(
            "Error when trying to open an URL in your browser: {:?}. \
             Please navigate here manually: {}",
            why,
            url
        ),
    }

    spotify
}


//! auth.rs
//! performs authentication with spotify

use std::env;

use rspotify::{
    prelude::*,
    scopes, AuthCodePkceSpotify, Credentials, OAuth,
};

const RSPOTIFY_CLIENT_ID: &str = "291fcee697f94999bf71fbf499bdfb54";
const RSPOTIFY_CLIENT_SECRET: &str = "439e720432e149c0bb3176a3b5e1713c";
const RSPOTIFY_REDIRECT_URI: &str = "http://localhost:8888/callback";

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

pub async fn auth_code_pkce_flow() -> AuthCodePkceSpotify {
    _set_env_var();

    let creds = creds();
    let oauth = oauth();

    let mut spotify = AuthCodePkceSpotify::new(creds.clone(), oauth.clone());

    let url = spotify.get_authorize_url(None).unwrap();

    spotify.prompt_for_token(&url).await.unwrap();

    let history = spotify.current_playback(None, None::<Vec<_>>).await;
    println!("Response: {history:?}");

    spotify
}

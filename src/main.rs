use rspotify::{
    model::{AdditionalType, ArtistId, Country, Market, UserId},
    prelude::*,
    scopes, AuthCodeSpotify, ClientCredsSpotify, ClientError, ClientResult, Config, Credentials,
    OAuth,
};

use std::env;

use log::{debug, error, info};
use log4rs::append::console::ConsoleAppender;
use log4rs::append::file::FileAppender;
use log4rs::config::{Appender, Logger, Root};
use log4rs::encode::pattern::PatternEncoder;
use log4rs::Handle;

#[tokio::main]
async fn main() {
    // You can use any logger for debugging.

    let stdout = ConsoleAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)(utc)} - {h({l})}: {m}{n}",
        )))
        .build();

    // Create a file appender
    let requests = FileAppender::builder()
        .encoder(Box::new(PatternEncoder::new(
            "{d(%Y-%m-%d %H:%M:%S)(utc)} - {h({l})}: {m}{n}",
        )))
        .build("log/requests.log")
        .expect("Failed to create file appender");

    // Build the log4rs configuration
    let config = log4rs::Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .appender(Appender::builder().build("requests", Box::new(requests)))
        .logger(Logger::builder().build("app::backend::db", log::LevelFilter::Info))
        .logger(
            Logger::builder()
                .appender("requests")
                .additive(false)
                .build("app::requests", log::LevelFilter::Debug),
        )
        .build(
            Root::builder()
                .appender("stdout")
                .appender("requests")
                .build(log::LevelFilter::Debug),
        )
        .unwrap();

    // use handle to change logger configuration at runtime
    let _handle = log4rs::init_config(config).expect("Failed to initialize log4rs");

    // Set the log4rs handle as the global logger
    log::set_max_level(log::LevelFilter::Debug);

    // Example log statements
    error!("This is an error message");
    info!("This is an info message");

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

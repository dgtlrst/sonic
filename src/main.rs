//! main.rs
//! sonic main

// libs
mod auth;
mod logging;

#[tokio::main]
async fn main() {
    // Initialize logging
    let _handle = logging::log_init();

    // log example
    log::info!("Starting Sonic");

    // Perform PKCE authentication flow
    let code = auth::auth_code_pkce_flow().await;
}

//! main.rs
//! sonic main

// TODO: the app will contain a minimal REPL that will be taking user input
    // and performing actions (displaying options) based on a predefined list (menu)
    // minimal functionalities required:
    // - display menu
    // - display current state
    // - poll for playlists and display user profile metadata
    // - provide option for exporting playlists to a file
    // - provide option for importing playlists from a file

use std::io::Write;
use tokio::signal;

// libs
mod auth;
mod logging;
mod backend;

#[tokio::main]
async fn main() {
    // Initialize logging
    let _handle = logging::log_init();

    // log example
    log::info!("Starting Sonic");

    // Spawn a task to handle SIGINT (Ctrl+C)
    backend::spawn_cleanup_task().await;

    // Start backend server
    backend::run_daemon().await;

    // Perform PKCE authentication flow + init tcp server/listener (separate that)
    let _ = auth::auth_code_pkce_flow().await;

    // Request token

    loop {
        // check if daemon is running
        println!("daemon is running [true]");
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }
}

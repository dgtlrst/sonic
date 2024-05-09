//! backend.rs
//! backend server handling various requests

use tokio::signal;
use warp::Filter;
use std::{collections::HashMap, io::Write};

pub async fn spawn_cleanup_task() {
    let sigint = signal::ctrl_c();
    tokio::spawn(async move {
        sigint.await.unwrap();
        println!("Received SIGINT, shutting down gracefully...");
        // You can add cleanup logic here if necessary
        // Add cleanup logic here
        // ...

        // Exit the program
        // flush stdout and stderr
        std::io::stdout().flush().unwrap();
        std::process::exit(0);
    });
}

pub async fn run_daemon() -> () {
    // Create a TCP listener bound to localhost:8080
    let port = 8888;
    let addr = format!("127.0.0.1/{}", port);

    println!("Server running on {}", addr);

    // spawn async task to handle connections
    tokio::spawn(async move {

        let callback = warp::path("callback")
            .and(warp::get())
            .and(warp::filters::query::query())
            .map(callback_handler);

        // TODO: implement the routes for path()
        // TODO: add 'health' endpoint to check if daemon is running and perform backend diagnostics if necessary
        let routes = callback.with(warp::log("auth"));

        // start server and return handle
        let (addr, server) = warp::serve(routes.clone()).bind_with_graceful_shutdown(([127, 0, 0, 1], port), async {
            signal::ctrl_c().await.unwrap();
        });
        println!("Server listening on http://{}", addr);
        server.await;
        // warp::serve(routes.clone()).run(([127, 0, 0, 1], 8888)).await
    });
}

fn callback_handler(query: HashMap<String, String>) -> impl warp::Reply {
    println!("Query: {:#?}", query);
    // TODO: handle the query + add protection!!
    warp::reply::html("Callback received")
}

use std::net::SocketAddr;
use tokio::net::TcpListener;
use tokio::io::{AsyncReadExt, AsyncWriteExt};

#[tokio::main]
async fn main() {
    // Create a TCP listener bound to localhost:8080
    let addr = "127.0.0.1:8080".parse::<SocketAddr>().unwrap();
    let listener = TcpListener::bind(&addr).await.unwrap();
    println!("Server running on {}", addr);

    loop {
        // Accept a new TCP connection
        let (mut socket, _) = listener.accept().await.unwrap();

        // Spawn a new async task to handle the connection
        tokio::spawn(async move {
            // Read the HTTP request
            let mut buf = [0; 1024];
            let n = socket.read(&mut buf).await.unwrap();

            // Write a simple HTTP response
            let response = "HTTP/1.1 200 OK\r\nContent-Length: 12\r\n\r\nHello World!";
            socket.write_all(response.as_bytes()).await.unwrap();

            // Shutdown the socket
            socket.shutdown().await.unwrap();
        });
    }
}


let mut handles = Vec::new();
    let port = 3030;
    for i in 0..12 {
        let routes_clone = routes.clone();
        let handle = tokio::task::spawn(async move {
            let tcp = TcpBuilder::new_v4().unwrap();
            let listener = tcp.reuse_address(true).unwrap()
                .reuse_port(true).unwrap()
                .bind(format!("0.0.0.0:{}", port + i)).unwrap()
                .listen(10000).unwrap();

            listener.set_nonblocking(true).unwrap();

            let mut listener = TcpListener::from_std(listener).unwrap();
            let stream = listener.incoming();
            warp::serve(routes_clone.clone()).run_incoming(stream).await
        });

        handles.push(handle);
    }
    for handle in handles {
        handle.await.unwrap();
    }
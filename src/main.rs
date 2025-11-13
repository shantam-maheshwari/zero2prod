use std::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:8005").expect("Failed to bind port 8005");
    let server = zero2prod::run(listener).expect("Failed to spawn server.");
    server.await
}

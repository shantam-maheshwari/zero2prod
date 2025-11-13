use std::net::TcpListener;

use reqwest::Client;

#[tokio::test]
async fn health_check_works() {
    let address = spawn_app();
    let client = Client::new();

    let response = client
        .get(format!("{address}/health_check"))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());
    assert_eq!(response.content_length(), Some(0));
}

fn spawn_app() -> String {
    let listener = TcpListener::bind("127.0.0.1:0").expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();

    let server = zero2prod::run(listener).expect("Failed to bind address.");
    tokio::spawn(server);

    format!("http://127.0.0.1:{port}")
}

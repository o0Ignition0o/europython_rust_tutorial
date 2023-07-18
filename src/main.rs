use axum::{routing::get, routing::post, Router, Json, extract};
use std::net::SocketAddr;
use serde::{Serialize, Deserialize};

async fn hello() -> String {
    "Hello, World!".to_string()
}

#[derive(Serialize)]
struct HelloJson {
    sender: String,
    receiver: String,
    message: String
}

#[derive(Deserialize)]
struct HelloPayload {
    sender: String,
    receiver: String,
}

async fn say_hello_json(
    extract::Json(hello_payload): extract::Json<HelloPayload>,
) -> Json<HelloJson> {
    let message = format!(
        "{} says hello to {}",
        hello_payload.sender, hello_payload.receiver
    );
    Json(HelloJson {
        sender: hello_payload.sender,
        receiver: hello_payload.receiver,
        message,
    })
}

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(hello)).route("/json", post(say_hello_json));

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

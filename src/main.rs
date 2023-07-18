use axum::{
    extract::{self, Path, State},
    response::Html,
    routing::get,
    routing::post,
    Json, Router,
};
use serde::{Deserialize, Serialize};

use std::{
    net::SocketAddr,
    sync::{atomic::AtomicUsize, Arc},
};

#[derive(Deserialize)]
struct HelloPayload {
    sender: String,
    receiver: String,
}

// curl --request POST \
//     --header 'content-type: application/json' \
//     --url http://localhost:3000/ \
//     --data '{"sender": "Jeremy", "receiver":"Paul"}'

#[tokio::main]
async fn main() {
    let app_state = Arc::new(AppState {
        view_counter: Default::default(),
    });
    let app = Router::new()
        .route("/", get(hello).post(say_hello))
        .route("/json", post(say_hello_json))
        .route("/hello/:person", get(say_hello_to_someone))
        .route("/views", get(count_views))
        .with_state(app_state);

    // run it
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

// curl http://localhost:3000
// Hello, World!
async fn hello() -> &'static str {
    "Hello, World!"
}

// curl --request POST \
//     --header 'content-type: application/json' \
//     --url http://localhost:3000/hello/jeremy \
//     --data '{"sender": "Jeremy", "receiver":"Paul"}'
async fn say_hello_to_someone(Path(person): Path<String>) -> String {
    format!("Hello, {}!", person)
}

// curl --request POST \
//     --header 'content-type: application/json' \
//     --url http://localhost:3000/ \
//     --data '{"sender": "Jeremy", "receiver":"Paul"}'
async fn say_hello(extract::Json(hello_payload): extract::Json<HelloPayload>) -> String {
    format!(
        "{} says hello to {}",
        hello_payload.sender, hello_payload.receiver
    )
}

#[derive(Serialize)]
struct HelloJson {
    sender: String,
    receiver: String,
    message: String,
}

// curl --request POST \
//     --header 'content-type: application/json' \
//     --url http://localhost:3000/json \
//     --data '{"sender": "Jeremy", "receiver":"Paul"}'
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

struct AppState {
    view_counter: AtomicUsize,
}

// curl http://localhost:3000/views
// 1
async fn count_views(State(state): State<Arc<AppState>>) -> String {
    let previous_count = state
        .view_counter
        .fetch_add(1, std::sync::atomic::Ordering::SeqCst);

    format!("this page was viewed {} times.", previous_count)
}

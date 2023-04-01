use axum::{routing::get, routing::post, Json, Router,response::Html,extract::Query};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new().
    route("/", get(handler))
    .route("/foo", get(second_handler))
    .route("/bar", post(post_param_with_multi_values));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));

    println!("Server started, listening on {addr}");
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start server");
}

#[derive(serde::Serialize)]
struct Message {
    message: String,
}

async fn handler() -> Html<&'static str> {
    Html("<h1>Hello, World!</h1>")
}

async fn second_handler() -> Json<Message> {
    Json(Message {
        message: String::from("Hello, World2!"),
    })
}

async fn post_param_with_multi_values(
    Query(params): Query<Vec<(String, String)>>
) -> String {
    format!("params: {:?}", params)
}
use axum::{extract::Query, response::Html, routing::get, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;


#[derive(OpenApi)]
#[openapi(
    paths(
        
    ),
    components(
        schemas(QueryParams)
    ),
    tags(
        (name = "todo", description = "Todo items management API")
    )
)]
struct ApiDoc;

#[utoipa::path(
    post,
    path = "/foo",
    request_body = Todo,
    responses(
        (status = 201, description = "Todo item created successfully", body = Todo),
        (status = 409, description = "Todo already exists", body = TodoError)
    )
)]

#[tokio::main]
async fn main() {
    
    //
    let swagger = SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi());
    
    let app = Router::new()
    .route("/", get(handler))
    .route("/foo", get(second_handler))
    .route("/bar", post(query_params))
    .merge(swagger);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3030));

    println!("Server now listening on {addr}");

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("Failed to start the server");
}

// handlers

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

#[derive(Serialize, Deserialize,ToSchema)]
struct QueryParams {
    message: String,
    id: i32,
}

async fn query_params(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    Json(query)
}

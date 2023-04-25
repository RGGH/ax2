use axum::{extract::Query, response::Html, routing::get, routing::post, Json, Router};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

use utoipa::{OpenApi, ToSchema};
use utoipa_swagger_ui::SwaggerUi;

#[derive(OpenApi)]
#[openapi(
    paths(

        query_params,
        second_handler

    ),
    components(
        schemas(QueryParams)
    ),
    tags(
        (name = "AX2", description = "Special Management API")
    )
)]
struct ApiDoc;


#[tokio::main]
async fn main() {
    let swagger = SwaggerUi::new("/swagger-ui").url("/api-doc/openapi.json", ApiDoc::openapi());

    let app = Router::new()
        .route("/", get(handler))
        .route("/foo", get(second_handler))
        .route("/query", post(query_params))
        .merge(swagger);

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));

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

#[utoipa::path(
    get,
    path="/foo",
    responses(
        (status = 201, description = "item created successfully"),
        (status = 409, description = "already exists")
    )
)]
async fn second_handler() -> Json<Message> {
    Json(Message {
        message: String::from("Hello, World2!"),
    })
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct QueryParams {
    message: String,
    id: i32,
}

#[utoipa::path(
    post, get,
    path="/query",
    responses(
        (status = 201, description = "item created successfully"),
        (status = 409, description = "already exists")
    )
)]
pub async fn query_params(Query(query): Query<QueryParams>) -> Json<QueryParams> {
    Json(query)
}

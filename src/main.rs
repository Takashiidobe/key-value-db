use axum::{
    routing::{post, get, put, delete},
    http::StatusCode,
    http::Method,
    response::IntoResponse,
    Json, Router, extract::Path
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use uuid::Uuid;
use serde_json::value::Value;
use kv::{Store, Config};
use tower_http::cors::{Any, CorsLayer};
use std::env;

fn get_bucket() -> kv::Bucket<'static, String, String> {
    let cfg: Config = Config::new("./kv_data");
    let store: Store = Store::new(cfg).unwrap();

    store.bucket::<String, String>(None).unwrap()
}


#[tokio::main]
async fn main() {
let cors = CorsLayer::new()
    .allow_methods(vec![Method::GET, Method::POST, Method::PUT, Method::DELETE])
    .allow_origin(Any)
    .allow_headers(Any);

    let port = env::var("PORT").unwrap_or("8367".to_string());
    let app = Router::new()
        .layer(cors)
        .route("/:id", get(query_val))
        .route("/:id", put(update_val))
        .route("/:id", delete(delete_val))
        .route("/", post(create_val));

    let addr = SocketAddr::from(([127, 0, 0, 1], port.parse::<u16>().unwrap()));
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn delete_val(
    Path(payload): Path<QueryVal>,
) -> impl IntoResponse {
    let bucket = get_bucket();
    let id = payload.id.to_owned();

    bucket.remove(&id).unwrap();

    StatusCode::NO_CONTENT
}

async fn update_val(
    Path(path): Path<QueryVal>,
    Json(payload): Json<CreateVal>,
) -> impl IntoResponse {
    let id = path.id;
    let value = payload.value;
    let str_val = serde_json::to_string(&value).unwrap();
    let bucket = get_bucket();

    bucket.set(&id, &str_val).unwrap();

    (StatusCode::OK, Json(Val { id, value }))
}

async fn create_val(
    Json(payload): Json<CreateVal>,
) -> impl IntoResponse {
    let id = Uuid::new_v4().to_hyphenated().to_string();
    let value = payload.value;
    let str_val = serde_json::to_string(&value).unwrap();
    let bucket = get_bucket();

    bucket.set(&id, &str_val).unwrap();

    (StatusCode::CREATED, Json(Val { id, value }))
}

async fn query_val(
    Path(payload): Path<QueryVal>,
) -> impl IntoResponse {
    let bucket = get_bucket();
    let id = payload.id.to_owned();
    let value = serde_json::from_str(&bucket.get(&id).unwrap().unwrap_or(String::default())).unwrap_or(Value::Null);

    (StatusCode::OK, Json(Val { id, value }))
}

#[derive(Deserialize, PartialEq)]
struct CreateVal {
    value: Value,
}

#[derive(Deserialize, Serialize, PartialEq, Debug)]
struct QueryVal {
    id: String,
}

#[derive(Serialize, PartialEq)]
struct Val {
    id: String,
    value: Value,
}

use std::str::FromStr;

pub use crate::*;
use axum::extract::Json;
use axum::extract::Path;
use axum::extract::State;
use axum::routing::{delete, get, patch, post, RouterIntoService};
use axum::Router;
use chrono::Local;
use hyper::StatusCode;
use serde_json::Value;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub async fn construct_routes(db_store: Store) -> Router {
    let app = Router::new()
        .route("/", get(root))
        .route("/posts/all", get(get_all_posts))
        .route("/posts/:id", get(get_single_post))
        .route("/posts/new", post(create_post_handler))
        .route("/posts/:id", patch(amend_post))
        .route("/posts/:id", delete(delete_post_handler))
        .route("/api/healthcheck", get(health_check))
        .with_state(db_store);
    return app;
}

// basic handler that responds with a static string
pub async fn root(State(db_store): State<Store>) -> &'static str {
    "Hello, World!"
}

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn get_all_posts(State(db_store): State<Store>) {
    let vec_posts: Vec<Post> = db_store.get_all().await.unwrap();
    dbg!(vec_posts);
}

pub async fn get_single_post(State(db_store): State<Store>, Path(id): Path<String>) {
    dbg!(&id);
    let post: Post = db_store.get_by_id(id).await.unwrap();
    dbg!(post);
}

pub async fn create_post_handler(State(db_store): State<Store>, Json(payload): Json<Post>) {
    dbg!(&payload);
    //let post: Post = serde_json::from_value(payload).unwrap();
    db_store.create_post(payload).await.unwrap();
    dbg!("Post created");
}

pub async fn amend_post(
    State(db_store): State<Store>,
    Path(id): Path<String>,
    Json(payload): Json<Post>,
) {
    dbg!(&payload);
    db_store.update_post(id, payload).await.unwrap();
}

pub async fn delete_post_handler(
    State(db_store): State<Store>,
    Path(id): Path<String>,
    Json(payload): Json<Post>,
) {
    dbg!(&payload);
    db_store.delete_post(id).await.unwrap();
    //    db_store.delete_post(post.post_id).await.unwrap();
}

pub use crate::*;
use axum::extract::Path;
use axum::extract::State;
use axum::routing::{delete, get, patch, post, RouterIntoService};
use axum::Router;
use hyper::StatusCode;
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
    db_store.get_all().await.unwrap();
}

pub async fn get_single_post(State(db_store): State<Store>, Path(id): Path<Uuid>) {
    db_store.get_by_id(id).await.unwrap();
}

pub async fn create_post_handler(State(db_store): State<Store>, body: String) {
    dbg!(body);
}

pub async fn amend_post(State(db_store): State<Store>, body: String) {
    dbg!(body);
}

pub async fn delete_post_handler(State(db_store): State<Store>, body: String) {
    dbg!(body);
}

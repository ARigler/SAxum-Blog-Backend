use std::str::FromStr;
use std::sync::Arc;

pub use crate::*;
use axum::extract::{Json, Path, State};
use axum::middleware;
use axum::response::IntoResponse;
use axum::routing::{delete, get, patch, post, RouterIntoService};
use axum::Router;
use chrono::Local;
use chrono::Utc;
use hyper::StatusCode;
use jsonwebtoken::encode;
use jsonwebtoken::Header;
use serde_json::Value;
use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

pub async fn construct_routes(db_store: Store) -> Router {
    let db_arc = Arc::new(db_store);
    let app = Router::new()
        .route("/", get(root))
        .route("/posts/all", get(get_all_posts))
        .route("/posts/:id", get(get_single_post))
        .route(
            "/posts/new",
            post(create_post_handler).route_layer(middleware::from_fn_with_state(
                db_arc.clone(),
                auth::authorization_middleware,
            )),
        )
        .route(
            "/posts/:id",
            patch(amend_post).route_layer(middleware::from_fn_with_state(
                db_arc.clone(),
                auth::authorization_middleware,
            )),
        )
        .route(
            "/posts/:id",
            delete(delete_post_handler).route_layer(middleware::from_fn_with_state(
                db_arc.clone(),
                auth::authorization_middleware,
            )),
        )
        .route("/signin", post(auth::sign_in))
        .route(
            "/users/new",
            post(create_user_handler).route_layer(middleware::from_fn_with_state(
                db_arc.clone(),
                auth::authorization_middleware,
            )),
        )
        .route("/users", get(get_all_users))
        .route(
            "/users/:id",
            delete(delete_user_handler).route_layer(middleware::from_fn_with_state(
                db_arc.clone(),
                auth::authorization_middleware,
            )),
        )
        .route("/api/healthcheck", get(health_check))
        .with_state(db_arc.clone());
    return app;
}

// basic handler that responds with a static string
pub async fn root(State(db_store): State<Arc<Store>>) -> &'static str {
    "Hello, World!"
}

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}

pub async fn get_all_posts(State(db_store): State<Arc<Store>>) -> impl IntoResponse {
    let vec_posts: Vec<Post> = db_store.get_all().await.unwrap();
    let json_response = serde_json::json!({
        "status": "success".to_string(),
        "posts": vec_posts
    });
    Json(json_response)
}

pub async fn get_single_post(
    State(db_store): State<Arc<Store>>,
    Path(id): Path<String>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let post: Post = db_store.get_by_id(id).await.unwrap();

    return Ok((StatusCode::OK, Json(post)));
}

pub async fn create_post_handler(
    State(db_store): State<Arc<Store>>,
    Json(payload): Json<Post>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    let mut post_new = payload.clone();
    post_new.post_date = Datetime(Utc::now());

    if let Ok(post) = db_store.get_by_title(payload.post_title.clone()).await {
        let json_response = serde_json::json!({
            "status": "error".to_string(),
            "message": "Entry already exists".to_string(),
            "data": post,
        });
        return Err((StatusCode::BAD_REQUEST, Json(json_response)));
    }
    db_store.create_post(post_new.clone()).await.unwrap();
    let json_response = serde_json::json!({
        "status": "success".to_string(),
        "data": post_new,
    });
    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn amend_post(
    State(db_store): State<Arc<Store>>,
    Path(id): Path<String>,
    Json(payload): Json<Post>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    match db_store.get_by_id(id.clone()).await {
        Ok(post) => {
            let title = payload.post_title.to_owned();
            let content = payload.post_body.to_owned();
            let posterid = payload.poster_id.to_owned();
            let postdate = payload.post_date.to_owned();
            let edited_post = Post {
                id: None,
                poster_id: posterid,
                post_title: title,
                post_date: postdate,
                post_body: content,
            };
            let post_response = db_store.update_post(id, edited_post).await.unwrap();
            let json_response = serde_json::json!({
                "status": "success",
                "data": post_response,
            });
            Ok((StatusCode::OK, Json(json_response)))
        }
        Err(_) => {
            let error_response = serde_json::json!({
                "status": "error",
                "message": format!("Entry with ID: {} not found",id)
            });
            Err((StatusCode::NOT_FOUND, Json(error_response)))
        }
    }
}

pub async fn delete_post_handler(
    State(db_store): State<Arc<Store>>,
    Path(id): Path<String>,
    Json(payload): Json<Post>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if let Ok(_) = db_store.get_by_id(id.clone()).await {
        let _ = db_store.delete_post(id.clone()).await.unwrap();
        return Ok(StatusCode::NO_CONTENT);
    }
    let error_response = serde_json::json!({
        "status": "Error".to_string(),
        "data": format!("Post with ID: {} not found",id)
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

pub async fn create_user_handler(
    State(db_store): State<Arc<Store>>,
    Json(payload): Json<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if let Ok(user) = db_store.retrieve_user_by_email(payload.email.clone()).await {
        let json_response = serde_json::json!({
            "status": "error".to_string(),
            "message": "User already exists".to_string(),
            "data": user,
        });
        return Err((StatusCode::BAD_REQUEST, Json(json_response)));
    }
    db_store.add_user(payload.clone()).await.unwrap();
    let json_response = serde_json::json!({
        "status": "success".to_string(),
        "data": payload,
    });
    Ok((StatusCode::CREATED, Json(json_response)))
}

pub async fn get_all_users(State(db_store): State<Arc<Store>>) -> impl IntoResponse {
    let vec_users: Vec<User> = db_store.get_users().await.unwrap();
    let json_response = serde_json::json!({
        "status": "success".to_string(),
        "users": vec_users
    });
    Json(json_response)
}

pub async fn delete_user_handler(
    State(db_store): State<Arc<Store>>,
    Path(id): Path<String>,
    Json(payload): Json<User>,
) -> Result<impl IntoResponse, (StatusCode, Json<serde_json::Value>)> {
    if let Ok(_) = db_store.get_user_by_id(id.clone()).await {
        let _ = db_store.delete_user(id.clone()).await.unwrap();
        return Ok(StatusCode::NO_CONTENT);
    }
    let error_response = serde_json::json!({
        "status": "Error".to_string(),
        "data": format!("User with ID: {} not found",id)
    });
    Err((StatusCode::NOT_FOUND, Json(error_response)))
}

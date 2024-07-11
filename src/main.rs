use ::serde::{Deserialize, Serialize};
use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use axum_crud::construct_routes;
use axum_crud::model::*;
use axum_crud::store::*;
use chrono::prelude::*;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();

    // build our application with a route
    let repository = Store::new().await;
    let app = construct_routes(repository).await;

    // run our app with hyper, listening globally on port 3000
    let server_port = "0.0.0.0:3000";
    println!("Listening on {}", server_port);
    let listener = tokio::net::TcpListener::bind(server_port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

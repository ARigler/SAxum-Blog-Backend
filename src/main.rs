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
use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    // initialize tracing
    tracing_subscriber::fmt::init();
    dotenv().unwrap();
    // build our application with a route
    let repository = Store::new().await;
    let app = construct_routes(repository).await;

    // run our app with hyper, listening globally on port 3000
    let server_config = ServerConfig {
        host: env::var("HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
        port: env::var("PORT").unwrap_or_else(|_| String::from("3000")),
    };
    let server_port = server_config.host + ":" + &server_config.port;
    println!("Listening on {}", server_port);
    let listener = tokio::net::TcpListener::bind(server_port).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Debug)]
pub struct ServerConfig {
    host: String,
    port: String,
}

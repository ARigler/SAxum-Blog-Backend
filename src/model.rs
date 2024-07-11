use ::serde::{Deserialize, Serialize};
use chrono::prelude::*;
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::*;
use surrealdb::{Error, Surreal};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Post {
    pub id: Option<Thing>,
    pub poster_id: Uuid,
    pub post_title: String,
    pub post_date: Datetime,
    pub post_body: String,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct User {
    pub user_id: Uuid,
    pub user_name: String,
    pub password: String,
}

use ::serde::{Deserialize, Serialize};
use surrealdb::engine::remote::ws::Ws;
use surrealdb::opt::auth::Root;
use surrealdb::sql::*;
use surrealdb::{Error, Surreal};

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct Post {
    pub post_id: Uuid,
    pub post_title: String,
    pub post_date: Datetime,
    pub post_body: String,
}

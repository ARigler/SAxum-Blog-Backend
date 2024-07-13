use std::env;
use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::opt::auth::Root;
use surrealdb::{Result, Surreal};

pub async fn get_database() -> Result<Surreal<Client>> {
    let db: Surreal<Client> = Surreal::init();
    let db_config = DatabaseConfig {
        host: env::var("DB_HOST").unwrap_or_else(|_| String::from("127.0.0.1")),
        port: env::var("DB_PORT").unwrap_or_else(|_| String::from("8000")),
    };
    let _ = db
        .connect::<Ws>(db_config.host + ":" + &db_config.port)
        .await?;
    //Auth here
    //db.signin(Root {
    //    username: "root",
    //    password: "root",
    //})
    //.await?;
    //Auth done
    let _ = db.use_ns("posts").use_db("posts").await?;

    Ok(db)
}

#[derive(Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: String,
}

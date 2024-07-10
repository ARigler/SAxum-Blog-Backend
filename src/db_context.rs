use surrealdb::engine::remote::ws::{Client, Ws};
use surrealdb::{Result, Surreal};

pub async fn get_database() -> Result<Surreal<Client>> {
    let db: Surreal<Client> = Surreal::init();
    let _ = db.connect::<Ws>("127.0.0.1:8000").await?;
    //Auth here
    //db.signin(Root{
    //    username: "root",
    //    password: "root",
    //}).await?;
    //Auth done
    let _ = db.use_ns("posts").use_db("posts").await?;

    Ok(db)
}

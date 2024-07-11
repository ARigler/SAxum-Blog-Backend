use crate::*;
pub use surrealdb::engine::remote::ws::Client;
pub use surrealdb::error::Db::Thrown;
pub use surrealdb::sql::*;
pub use surrealdb::Error;
pub use surrealdb::Surreal;

#[derive(Clone)]
pub struct Store {
    table: String,
    db: Surreal<Client>,
}

impl Store {
    pub async fn new() -> Self {
        let db = get_database().await.unwrap();
        Store {
            table: String::from("posts"),
            db,
        }
    }

    pub async fn get_all(&self) -> Result<Vec<Post>, Error> {
        let records = self.db.select(&self.table).await?;
        Ok(records)
    }

    pub async fn get_by_id(&self, id: String) -> Result<Post, Error> {
        if let Some(record) = self.db.select((&self.table, id.clone())).await? {
            return Ok(record);
        }
        let error = Error::Db(Thrown(format!("Record with id {} not found", id)));
        Err(error)
    }

    pub async fn create_post(&self, content: Post) -> Result<Vec<Post>, Error> {
        let record = self.db.create(&self.table).content(content).await?;
        Ok(record)
    }

    pub async fn update_post(&self, id: String, content: Post) -> Result<Post, Error> {
        let record = self
            .db
            .update((&self.table, id))
            .content(content)
            .await?
            .unwrap();
        Ok(record)
    }

    pub async fn delete_post(&self, id: String) -> Result<Post, Error> {
        let result = self.db.delete((&self.table, id)).await?.unwrap();
        Ok(result)
    }
}

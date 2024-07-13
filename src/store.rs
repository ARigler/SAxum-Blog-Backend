use crate::*;
pub use surrealdb::engine::remote::ws::Client;
pub use surrealdb::error::Db::Thrown;
pub use surrealdb::sql::*;
pub use surrealdb::Error;
pub use surrealdb::Surreal;

#[derive(Clone)]
pub struct Store {
    table: String,
    user_table: String,
    db: Surreal<Client>,
}

impl Store {
    pub async fn new() -> Self {
        let db = get_database().await.unwrap();
        Store {
            table: String::from("posts"),
            user_table: String::from("users"),
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

    pub async fn get_by_title(&self, title: String) -> Result<Post, Error> {
        if let Some(record) = self
            .db
            .query("SELECT * FROM posts WHERE post_title = $title")
            .bind(("title", title.clone()))
            .await?
            .take(0)?
        {
            return Ok(record);
        }
        let error = Error::Db(Thrown(format!("Post with title {} not found", title)));
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

    pub async fn add_user(&self, user: User) -> Result<Vec<User>, Error> {
        let mut new_user = user.clone();
        new_user.password = hash_password(&new_user.password).unwrap();
        let result = self.db.create(&self.user_table).content(new_user).await?;
        Ok(result)
    }

    pub async fn update_password(&self, id: String, new_user: User) -> Result<User, Error> {
        let mut newer_user = new_user.clone();
        newer_user.password = hash_password(&new_user.password).unwrap();
        let record = self
            .db
            .update((&self.user_table, id))
            .content(newer_user)
            .await?
            .unwrap();
        Ok(record)
    }

    pub async fn delete_user(&self, id: String) -> Result<User, Error> {
        let result = self.db.delete((&self.user_table, id)).await?.unwrap();
        Ok(result)
    }

    pub async fn retrieve_user_by_email(&self, email: String) -> Result<User, Error> {
        if let Some(record) = self
            .db
            .query("SELECT * FROM users WHERE email = $emailbind")
            .bind(("emailbind", email.clone()))
            .await?
            .take(0)?
        {
            dbg!(&record);
            return Ok(record);
        }
        let error = Error::Db(Thrown(format!("User not found")));
        dbg!(&error);
        Err(error)
    }

    pub async fn get_users(&self) -> Result<Vec<User>, Error> {
        let records = self.db.select(&self.user_table).await?;
        Ok(records)
    }

    pub async fn get_user_by_id(&self, id: String) -> Result<User, Error> {
        if let Some(record) = self.db.select((&self.user_table, id.clone())).await? {
            return Ok(record);
        }
        let error = Error::Db(Thrown(format!("User with id {} not found", id)));
        Err(error)
    }
}

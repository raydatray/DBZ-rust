pub(crate) mod practice;
pub(crate) mod user;

use std::env;

use mongodb::{bson::doc, Client, Database};
use tracing::info;

use crate::models::user::User;

pub(crate) struct DB {
    client: Client,
    database: Database,
}

impl DB {
    const USER_COLLECTION: &'static str = "users";
    const PRACTICE_COLLECITON: &'static str = "practices";

    pub(crate) async fn init() -> Result<Self, anyhow::Error> {
        info!("initing DB connection");

        let username = env::var("MONGO_USERNAME").expect("MONGO_USERNAME must be set");

        let password = env::var("MONGO_PASSWORD").expect("MONOG_PASSWORD must be set");

        let host = env::var("MONGO_HOST").unwrap_or_else(|_| {
            info!("MONGO_HOST not found in env, defaulting to localhost");
            "localhost".to_string()
        });

        let port = env::var("MONGO_PORT").unwrap_or_else(|_| {
            info!("MONGO_PORT not found in env, defaulting to 27017");
            "27017".to_string()
        });

        let db_name = env::var("MONGO_DB_NAME").unwrap_or_else(|_| {
            info!("MONGO_DB_NAME not found in env, defaulting to DBZ");
            "DBZ".to_string()
        });

        let mongo_uri = format!(
            "mongodb://{}:{}@{}:{}/{}?authSource=admin",
            username, password, host, port, db_name
        );

        info!("connecting to mongodb using: {}", mongo_uri);

        let client = Client::with_uri_str(&mongo_uri).await?;
        let database = client.database(&db_name);

        database.run_command(doc! {"ping" : 1}).await?;

        info!("connected to mongodb");

        Ok(Self { client, database })
    }
}

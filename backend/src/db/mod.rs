pub(crate) mod practice;
pub(crate) mod tenant;
pub(crate) mod user;

use std::env;

use mongodb::{bson::doc, Client, Database};
use tracing::info;

pub(crate) struct DB {
    client: Client,
    database: Database,
}

impl DB {
    const PRACTICE_COLLECITON: &'static str = "practices";
    const TENANT_COLLECTION: &'static str = "tenants";
    const USER_COLLECTION: &'static str = "users";

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

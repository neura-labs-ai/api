extern crate anyhow;

use mongodb::{Client, Database};

const DB_NAME: &str = "neuralabsai-dev";

#[derive(Clone, Debug)]
pub struct MongoDB {
    pub db_name: String,
    pub client: Client,
    pub db: Database,
}

impl MongoDB {
    pub async fn new(auth_url: String) -> anyhow::Result<Self> {
        let client = match Client::with_uri_str(auth_url).await {
            Ok(client) => client,
            Err(e) => return Err(anyhow::Error::new(e)),
        };

        let db = client.database(DB_NAME);

        Ok(Self {
            db_name: DB_NAME.to_string(),
            client,
            db,
        })
    }
}

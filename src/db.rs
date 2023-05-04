extern crate anyhow;

use mongodb::{
    bson::{doc, oid::ObjectId, Document},
    Client, Database,
};

use crate::models::{ReportStatus, SystemReport, Tokens, UserReport};

pub const DB_NAME: &str = "neuralabsai-dev";

#[derive(Clone, Debug)]
pub struct MongoDB {
    pub db_name: String,
    pub client: Client,
    pub db: Database,
}

pub enum CollectionNames {
    User,
    Account,
    Session,
    Tokens,
    SystemReport,
    UserReport,
    Statistics,
    Custom(String),
}

impl MongoDB {
    /// Initializes a new MongoDB instance
    pub async fn new(auth_url: &String) -> anyhow::Result<Self> {
        let client = match Client::with_uri_str(auth_url).await {
            Ok(client) => client,
            Err(e) => return Err(anyhow::Error::new(e)),
        };

        let db = client.database(DB_NAME);

        println!("MongoDB Initialized");

        Ok(Self {
            db_name: DB_NAME.to_string(),
            client,
            db,
        })
    }

    /// Checks if the MongoDB instance is alive
    ///
    /// Returns 1 if the instance is alive and 0 if it is not
    pub async fn ping(&self) -> anyhow::Result<Document> {
        match self.db.run_command(doc! {"ping": 1}, None).await {
            Ok(result) => Ok(result),
            Err(e) => Err(anyhow::Error::new(e)),
        }
    }

    /// Returns a MongoDB collection
    ///
    /// This is a helper function that also is typed
    pub fn get_collection<T>(&self, collection_name: CollectionNames) -> mongodb::Collection<T> {
        match collection_name {
            CollectionNames::User => self.db.collection("users"),
            CollectionNames::Account => self.db.collection("accounts"),
            CollectionNames::Session => self.db.collection("sessions"),
            CollectionNames::Tokens => self.db.collection("tokens"),
            CollectionNames::SystemReport => self.db.collection("system_reports"),
            CollectionNames::UserReport => self.db.collection("user_reports"),
            CollectionNames::Statistics => self.db.collection("statistics"),
            CollectionNames::Custom(name) => self.db.collection(&name),
        }
    }

    pub async fn create_system_report(
        &self,
        title: String,
        description: String,
    ) -> anyhow::Result<()> {
        let collection = self.get_collection::<SystemReport>(CollectionNames::SystemReport);

        let report = SystemReport {
            _id: ObjectId::new(),
            title,
            description,
            status: ReportStatus::InProgress,
            created_at: chrono::Utc::now(),
        };

        match collection.insert_one(report, None).await {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::Error::new(e)),
        }
    }

    pub async fn create_user_report(
        &self,
        title: String,
        description: String,
        assigned_to_id: Option<ObjectId>,
    ) -> anyhow::Result<()> {
        let collection = self.get_collection::<UserReport>(CollectionNames::UserReport);

        let report = UserReport {
            _id: ObjectId::new(),
            title,
            description,
            status: ReportStatus::InProgress,
            created_at: chrono::Utc::now(),
            assigned_to_id,
        };

        match collection.insert_one(report, None).await {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::Error::new(e)),
        }
    }

    // todo - on api startup, cache all current tokens in memory for faster access
    // todo - any new tokens will be added to the cache. This is to avoid querying the database for every request
    /// Checks if a api token exists in the database
    ///
    /// This is used to check if a token is valid. If it is, then the user is authenticated on the API.
    pub async fn has_api_key(&self, token: String) -> anyhow::Result<bool> {
        let collection = self.get_collection::<Tokens>(CollectionNames::Tokens);

        let filter = doc! {"token": token};

        match collection.find_one(filter, None).await {
            Ok(result) => match result {
                Some(_) => Ok(true),
                None => Ok(false),
            },
            Err(e) => Err(anyhow::Error::new(e)),
        }
    }

    /// Returns the API key for a given token
    pub async fn get_api_key(&self, token: String) -> anyhow::Result<String> {
        let collection = self.get_collection::<Tokens>(CollectionNames::Tokens);

        let filter = doc! {"token": token};

        match collection.find_one(filter, None).await {
            Ok(result) => match result {
                Some(data) => Ok(data.token),
                None => Err(anyhow::Error::msg("Token not found")),
            },
            Err(e) => Err(anyhow::Error::new(e)),
        }
    }

    /// Converts a string to an mongodb ObjectId
    pub fn convert_to_object_id(&self, id: String) -> anyhow::Result<ObjectId> {
        match ObjectId::parse_str(&id) {
            Ok(object_id) => Ok(object_id),
            Err(e) => Err(anyhow::Error::new(e)),
        }
    }
}

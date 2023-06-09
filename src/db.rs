extern crate anyhow;

use crate::models::{Credits, ReportStatus, Statistics, SystemReport, Tokens, Usage, UserReport};
use chrono::{DateTime, Utc};
use mongodb::{
    bson::{doc, oid::ObjectId, DateTime as BsonDateTime, Document, self},
    options::ReplaceOptions,
    Database, {Client, Collection},
};

pub const DB_NAME: &str = "neuralabsai";

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
    Credits,
    Payment,
    Statistics,
    SystemReport,
    UserReport,
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
    pub fn get_collection<T>(&self, collection_name: CollectionNames) -> Collection<T> {
        match collection_name {
            CollectionNames::User => self.db.collection("users"),
            CollectionNames::Account => self.db.collection("accounts"),
            CollectionNames::Session => self.db.collection("sessions"),
            CollectionNames::Tokens => self.db.collection("tokens"),
            CollectionNames::SystemReport => self.db.collection("system_reports"),
            CollectionNames::UserReport => self.db.collection("user_reports"),
            CollectionNames::Statistics => self.db.collection("statistics"),
            CollectionNames::Payment => self.db.collection("payments"),
            CollectionNames::Credits => self.db.collection("credits"),
            CollectionNames::Custom(name) => self.db.collection(&name),
        }
    }

    pub async fn create_system_report(
        &self,
        title: String,
        user_id: ObjectId,
        description: String,
    ) -> anyhow::Result<()> {
        let collection = self.get_collection::<SystemReport>(CollectionNames::SystemReport);

        let report = SystemReport {
            _id: ObjectId::new(),
            title,
            description,
            status: ReportStatus::InProgress,
            created_at: self.get_current_time().unwrap(),
            userId: user_id,
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
        assigned_to_id: ObjectId,
    ) -> anyhow::Result<()> {
        let collection = self.get_collection::<UserReport>(CollectionNames::UserReport);

        let report = UserReport {
            _id: ObjectId::new(),
            title,
            description,
            status: ReportStatus::InProgress,
            created_at: self.get_current_time().unwrap(),
            assignedToId: assigned_to_id,
        };

        match collection.insert_one(report, None).await {
            Ok(_) => Ok(()),
            Err(e) => Err(anyhow::Error::new(e)),
        }
    }

    /// Updates user credit information.
    ///
    /// This function is called when a user makes a request to the API and the request is successful.
    ///
    /// todo - Add the create_statistics_report function to this function. First we need to validate the current data and day.
    pub async fn process_credit_usage(&self, user_id: ObjectId) -> anyhow::Result<bool> {
        let collection = self.get_collection::<Credits>(CollectionNames::Credits);

        // Find an existing credits for the user.
        let filter = doc! {"userId": user_id};

        let mut credits = match collection.find_one(filter.clone(), None).await? {
            Some(credits) => credits,
            None => return Ok(false), // Return false if credits document not found.
        };

        // Check if user has enough credits.
        if credits.current_amount.unwrap_or(0) <= 0 {
            return Ok(false);
        }

        // Subtract one from current_amount and add one to used_amount.
        credits.current_amount = credits.current_amount.map(|amount| amount - 1);
        credits.used_amount = credits.used_amount.map(|amount| amount + 1);

        // Save the credits to the database.
        collection
            .replace_one(filter, credits.clone(), None)
            .await?;

        let u = Usage {
            api_calls: Some(1),
            api_calls_monday: None,
            api_calls_tuesday: None,
            api_calls_wednesday: None,
            api_calls_thursday: None,
            api_calls_friday: None,
            api_calls_saturday: None,
            api_calls_sunday: None,
            api_calls_success: None,
            api_calls_fail: None,
        };

        // Create a new statistics report for the user.
        self.create_statistics_report(user_id, Some(u)).await?;

        Ok(true)
    }

    // Creates a new stats report for the api.
    pub async fn create_statistics_report(
        &self,
        user_id: ObjectId,
        usage: Option<Usage>,
    ) -> anyhow::Result<()> {
        let collection = self.get_collection::<Statistics>(CollectionNames::Statistics);

        // Get the current date and time.
        let now = self.get_current_time().unwrap();

        // Find an existing statistics report for the user.
        let filter = doc! {"userId": user_id};
        let existing_report = collection.find_one(filter.clone(), None).await?;

        // Create a new statistics report or update the existing one.
        let report = match existing_report {
            Some(mut report) => {
                // Update the usage data if provided.
                if let Some(new_usage) = usage {
                    // Update the usage data if provided.
                    let mut old_usage = report.usage.take().unwrap();

                    // Increment the usage data if it exists, or set it to the new value if it doesn't.
                    old_usage.api_calls = old_usage
                        .api_calls
                        .map(|x| x + new_usage.api_calls.unwrap_or_default())
                        .or(new_usage.api_calls);
                    old_usage.api_calls_monday = old_usage
                        .api_calls_monday
                        .map(|x| x + new_usage.api_calls_monday.unwrap_or_default())
                        .or(new_usage.api_calls_monday);
                    old_usage.api_calls_tuesday = old_usage
                        .api_calls_tuesday
                        .map(|x| x + new_usage.api_calls_tuesday.unwrap_or_default())
                        .or(new_usage.api_calls_tuesday);
                    old_usage.api_calls_wednesday = old_usage
                        .api_calls_wednesday
                        .map(|x| x + new_usage.api_calls_wednesday.unwrap_or_default())
                        .or(new_usage.api_calls_wednesday);
                    old_usage.api_calls_thursday = old_usage
                        .api_calls_thursday
                        .map(|x| x + new_usage.api_calls_thursday.unwrap_or_default())
                        .or(new_usage.api_calls_thursday);
                    old_usage.api_calls_friday = old_usage
                        .api_calls_friday
                        .map(|x| x + new_usage.api_calls_friday.unwrap_or_default())
                        .or(new_usage.api_calls_friday);
                    old_usage.api_calls_saturday = old_usage
                        .api_calls_saturday
                        .map(|x| x + new_usage.api_calls_saturday.unwrap_or_default())
                        .or(new_usage.api_calls_saturday);
                    old_usage.api_calls_sunday = old_usage
                        .api_calls_sunday
                        .map(|x| x + new_usage.api_calls_sunday.unwrap_or_default())
                        .or(new_usage.api_calls_sunday);
                    old_usage.api_calls_success = old_usage
                        .api_calls_success
                        .map(|x| x + new_usage.api_calls_success.unwrap_or_default())
                        .or(new_usage.api_calls_success);
                    old_usage.api_calls_fail = old_usage
                        .api_calls_fail
                        .map(|x| x + new_usage.api_calls_fail.unwrap_or_default())
                        .or(new_usage.api_calls_fail);

                    // Update the usage field of the report struct.
                    report.usage = Some(old_usage);
                }

                // Update the updated_at field.
                report.updated_at = Some(now);

                report
            }
            None => {
                // Create a new statistics report.
                Statistics {
                    _id: ObjectId::new(),
                    created_at: now,
                    updated_at: None,
                    usage,
                    userId: user_id,
                }
            }
        };

        // Save the statistics report to the database.
        collection
            .replace_one(
                filter,
                report.clone(),
                ReplaceOptions::builder().upsert(true).build(),
            )
            .await?;

        Ok(())
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
    pub async fn get_api_key(&self, token: &str) -> anyhow::Result<Option<String>> {
        let collection = self.get_collection::<Tokens>(CollectionNames::Tokens);

        let filter = doc! {"token": token};

        match collection.find_one(filter, None).await {
            Ok(result) => match result {
                Some(data) => Ok(Some(data.token)),
                None => Ok(None),
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

    /// Gets the current DateTime in UTC
    /// This function converts the DateTime into MongoDB compatible format for storing in the database
    pub fn get_current_time(&self) -> anyhow::Result<bson::DateTime> {
        let now = Utc::now();
        let bson_date_time = BsonDateTime::from_chrono(now);
        Ok(bson_date_time)
    }
}

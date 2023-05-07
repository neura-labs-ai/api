// Database models for the application. This is a port from the web/prisma schema file in the
// Client side codebase.

use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub _id: ObjectId,
    pub name: Option<String>,
    pub username: Option<String>,
    pub bio: Option<String>,
    pub email: String,
    #[serde(with = "optional_iso8601")]
    pub email_verified: Option<DateTime<Utc>>,
    pub image: String,
    pub roles: Vec<UserRole>,
    pub telemetry: bool,
    pub tomestoned: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum UserRole {
    USER,
    CONTRIBUTOR,
    MODERATOR,
    ADMIN,
    SYSTEM,
    TEST,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tokens {
    pub _id: ObjectId,
    pub token: String,
    #[serde(with = "iso8601")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "optional_iso8601")]
    pub updated_at: Option<DateTime<Utc>>,
    pub tomestoned: bool,
    #[allow(non_snake_case)]
    pub userId: ObjectId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Statistics {
    pub _id: ObjectId,
    #[serde(with = "iso8601")]
    pub created_at: DateTime<Utc>,
    #[serde(with = "optional_iso8601")]
    pub updated_at: Option<DateTime<Utc>>,
    pub usage: Option<Usage>,
    #[allow(non_snake_case)]
    pub userId: ObjectId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Usage {
    pub api_calls: Option<i32>,
    pub api_calls_monday: Option<i32>,
    pub api_calls_tuesday: Option<i32>,
    pub api_calls_wednesday: Option<i32>,
    pub api_calls_thursday: Option<i32>,
    pub api_calls_friday: Option<i32>,
    pub api_calls_saturday: Option<i32>,
    pub api_calls_sunday: Option<i32>,
    pub api_calls_success: Option<i32>,
    pub api_calls_fail: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Credits {
    pub _id: ObjectId,
    pub used_amount: Option<i32>,
    pub current_amount: Option<i32>,
    #[allow(non_snake_case)]
    pub userId: ObjectId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Payment {
    pub _id: ObjectId,
    pub active: bool,
    pub subscription_id: String,
    #[serde(with = "iso8601")]
    pub subscription_date: DateTime<Utc>,
    #[serde(with = "iso8601")]
    pub subscription_end_date: DateTime<Utc>,
    pub subscription_cancelled: bool,
    #[serde(with = "optional_iso8601")]
    pub subscription_cancelled_date: Option<DateTime<Utc>>,
    pub subscription_cancelled_reason: Option<String>,
    pub credits_purchased: i32,
    #[allow(non_snake_case)]
    pub userId: ObjectId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SystemReport {
    pub _id: ObjectId,
    pub title: String,
    pub description: String,
    pub status: ReportStatus,
    #[serde(with = "iso8601")]
    pub created_at: DateTime<Utc>,
    #[allow(non_snake_case)]
    pub userId: ObjectId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserReport {
    pub _id: ObjectId,
    pub title: String,
    pub description: String,
    pub status: ReportStatus,
    #[serde(with = "iso8601")]
    pub created_at: DateTime<Utc>,
    #[allow(non_snake_case)]
    pub assignedToId: ObjectId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ReportStatus {
    InProgress,
    RESOLVED,
    CLOSED,
}

// Custom serialization for DateTime<Utc> to ISO8601
// We need this because prisma doesn't support DateTime<Utc> natively.
mod iso8601 {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    // Format from prisma - 2023-05-07T03:23:13.849+00:00
    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3fZ";

    pub fn serialize<S>(date: &DateTime<Utc>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let s = date.format(FORMAT).to_string();
        serializer.serialize_str(&s)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<DateTime<Utc>, D::Error>
    where
        D: Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        Utc.datetime_from_str(&s, FORMAT)
            .map_err(serde::de::Error::custom)
    }
}

// Allows us to use Option<DateTime<Utc>> in our models.
mod optional_iso8601 {
    use chrono::{DateTime, TimeZone, Utc};
    use serde::{self, Deserialize, Deserializer, Serializer};

    const FORMAT: &str = "%Y-%m-%dT%H:%M:%S%.3f%:z";

    pub fn serialize<S>(date: &Option<DateTime<Utc>>, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if let Some(date) = date {
            let s = date.format(FORMAT).to_string();
            serializer.serialize_str(&s)
        } else {
            serializer.serialize_none()
        }
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<Option<DateTime<Utc>>, D::Error>
    where
        D: Deserializer<'de>,
    {
        match Option::<String>::deserialize(deserializer)? {
            Some(s) => Utc
                .datetime_from_str(&s, FORMAT)
                .map(Some)
                .map_err(serde::de::Error::custom),
            None => Ok(None),
        }
    }
}

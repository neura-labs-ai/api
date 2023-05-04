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
pub struct Account {
    pub _id: ObjectId,
    pub user_id: ObjectId,
    pub r#type: String,
    pub provider: String,
    pub provider_account_id: String,
    pub refresh_token: Option<String>,
    pub access_token: Option<String>,
    pub expires_at: Option<i64>,
    pub token_type: Option<String>,
    pub scope: Option<String>,
    pub id_token: Option<String>,
    pub session_state: Option<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Session {
    pub _id: ObjectId,
    pub session_token: String,
    pub user_id: ObjectId,
    pub expires: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tokens {
    pub _id: ObjectId,
    pub token: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub tomestoned: bool,
    pub author_id: ObjectId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Statistics {
    pub _id: ObjectId,
    pub created_at: DateTime<Utc>,
    pub updated_at: Option<DateTime<Utc>>,
    pub usage: Option<Usage>,
    pub payments: Option<Vec<Payment>>,
    pub author_id: ObjectId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Usage {
    api_calls: Option<i32>,
    api_calls_day: Option<i32>,
    api_calls_week: Option<i32>,
    api_calls_month: Option<i32>,
    api_calls_year: Option<i32>,

    api_calls_success: Option<i32>,
    api_calls_fail: Option<i32>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Payment {
    pub active: bool,
    pub subscription_id: String,
    pub subscription_date: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SystemReport {
    pub _id: ObjectId,
    pub title: String,
    pub description: String,
    pub status: ReportStatus,
    pub created_at: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserReport {
    pub _id: ObjectId,
    pub title: String,
    pub description: String,
    pub status: ReportStatus,
    pub created_at: DateTime<Utc>,
    pub assigned_to_id: Option<ObjectId>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ReportStatus {
    InProgress,
    RESOLVED,
    CLOSED,
}

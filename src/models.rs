#[allow(unused_imports)]

// todo - fix datetime parsing errors

use chrono::{DateTime, Utc};
use mongodb::bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub id: ObjectId,
    pub name: Option<String>,
    pub username: Option<String>,
    pub bio: Option<String>,
    pub email: String,
    // pub email_verified: Option<DateTime<Utc>>,
    pub image: String,
    pub roles: Vec<UserRole>,
    pub telemetry: bool,
    pub tomestoned: bool,
    pub accounts: Vec<Account>,
    pub sessions: Vec<Session>,
    pub token: Vec<Tokens>,
    pub system_reports: Vec<SystemReport>,
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
    pub id: ObjectId,
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
    pub id: ObjectId,
    pub session_token: String,
    pub user_id: ObjectId,
    // pub expires: DateTime<Utc>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Tokens {
    pub id: ObjectId,
    pub token: String,
    // pub created_at: DateTime<Utc>,
    // pub updated_at: Option<DateTime<Utc>>,
    pub tomestoned: bool,
    pub author_id: ObjectId,
    pub owner: User,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SystemReport {
    pub id: ObjectId,
    pub title: String,
    pub description: String,
    pub status: ReportStatus,
    // pub created_at: DateTime<Utc>,
    pub assigned_to_id: Option<ObjectId>,
    pub assigned_to: Option<User>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub enum ReportStatus {
    InProgress,
    RESOLVED,
    CLOSED,
}
// Database models for the application. This is a port from the web/prisma schema file in the
// Client side codebase.

use mongodb::bson::{oid::ObjectId, self};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct User {
    pub _id: ObjectId,
    pub name: Option<String>,
    pub username: Option<String>,
    pub bio: Option<String>,
    pub email: String,
    pub email_verified: Option<bson::DateTime>,
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
    pub created_at: bson::DateTime,
    pub updated_at: Option<bson::DateTime>,
    pub tomestoned: bool,
    #[allow(non_snake_case)]
    pub userId: ObjectId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Statistics {
    pub _id: ObjectId,
    pub created_at: bson::DateTime,
    pub updated_at: Option<bson::DateTime>,
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
    pub subscription_date: bson::DateTime,
    pub subscription_end_date: bson::DateTime,
    pub subscription_cancelled: bool,
    pub subscription_cancelled_date: Option<bson::DateTime>,
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
    pub created_at: bson::DateTime,
    #[allow(non_snake_case)]
    pub userId: ObjectId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct UserReport {
    pub _id: ObjectId,
    pub title: String,
    pub description: String,
    pub status: ReportStatus,
    pub created_at: bson::DateTime,
    #[allow(non_snake_case)]
    pub assignedToId: ObjectId,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub enum ReportStatus {
    InProgress,
    RESOLVED,
    CLOSED,
}
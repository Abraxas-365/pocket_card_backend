use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub google_id: String,
    pub email: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl User {
    pub fn new(google_id: String, email: String) -> Self {
        Self {
            id: 0,
            google_id,
            email,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserProfile {
    pub id: String,
    pub user_id: i32,
    pub email: Option<String>,
    pub name: Option<String>,
    pub bio: Option<String>,
    pub phone_number: Option<String>,
    pub location: Option<String>,
    pub profile_picture_url: Option<String>,
    pub theme: Option<String>,
    pub custom_url: Option<String>,
    pub job_title: Option<String>,
    pub facebook_url: Option<String>,
    pub twitter_url: Option<String>,
    pub instagram_url: Option<String>,
    pub linkedin_url: Option<String>,
    pub updated_at: DateTime<Utc>,
}

impl UserProfile {
    pub fn new(user_id: i32) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            user_id,
            email: None,
            name: None,
            bio: None,
            phone_number: None,
            location: None,
            profile_picture_url: None,
            theme: None,
            custom_url: None,
            job_title: None,
            facebook_url: None,
            twitter_url: None,
            instagram_url: None,
            linkedin_url: None,
            updated_at: Utc::now(),
        }
    }
}

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct UserSetting {
    pub id: i32,
    pub user_id: i32,
    pub exchange_contacts: bool,
    pub save_contact: bool,
    pub call_me: bool,
    pub email_me: bool,
    pub social_media: bool,
    pub template: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl UserSetting {
    pub fn new(user_id: i32) -> Self {
        Self {
            id: 0,
            user_id,
            exchange_contacts: true,
            save_contact: true,
            call_me: true,
            email_me: true,
            social_media: false,
            template: "default".to_string(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }
}

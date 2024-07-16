use async_trait::async_trait;

use crate::error::ApiError;

use super::{User, UserProfile, UserSetting};

#[async_trait]
pub trait Repository: Sync + Send {
    async fn find_by_google_id(&self, google_id: &str) -> Result<User, ApiError>;
    async fn find_by_id(&self, id: i32) -> Result<User, ApiError>;
    async fn create(&self, user: &User) -> Result<User, ApiError>;
    async fn update(&self, user: &User) -> Result<User, ApiError>;

    async fn create_profile(&self, user: &UserProfile) -> Result<UserProfile, ApiError>;
    async fn update_profile(&self, user: &UserProfile) -> Result<UserProfile, ApiError>;
    async fn find_profile_by_user_id(&self, user_id: i32) -> Result<UserProfile, ApiError>;
    async fn find_profile_by_id(&self, id: &str) -> Result<UserProfile, ApiError>;

    async fn find_settings_by_user_id(&self, user_id: i32) -> Result<UserSetting, ApiError>;
    async fn find_settings_by_profile_id(&self, profile_id: &str) -> Result<UserSetting, ApiError>;
    async fn create_settings(&self, setting: &UserSetting) -> Result<UserSetting, ApiError>;
    async fn update_settings(&self, setting: &UserSetting) -> Result<UserSetting, ApiError>;
}

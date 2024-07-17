use super::{User, UserProfile};
use crate::error::ApiError;
use async_trait::async_trait;

#[async_trait]
pub trait Repository: Sync + Send {
    async fn find_by_google_id(&self, google_id: &str) -> Result<User, ApiError>;
    async fn find_by_id(&self, id: i32) -> Result<User, ApiError>;
    async fn create(&self, user: &User) -> Result<User, ApiError>;
    async fn update(&self, user: &User) -> Result<User, ApiError>;

    async fn create_profile(&self, profile: &UserProfile) -> Result<UserProfile, ApiError>;
    async fn update_profile(&self, profile: &UserProfile) -> Result<UserProfile, ApiError>;
    async fn find_profile_by_user_id(&self, user_id: i32) -> Result<UserProfile, ApiError>;
    async fn find_profile_by_id(&self, id: &str) -> Result<UserProfile, ApiError>;
}

use std::sync::Arc;

use crate::error::ApiError;

use super::{ports::Repository, User, UserProfile};

pub struct Service {
    user_repo: Arc<dyn Repository>,
}

impl Service {
    pub fn new(user_repo: Arc<dyn Repository>) -> Self {
        Self { user_repo }
    }
}

impl Service {
    pub async fn find_by_google_id(&self, google_id: &str) -> Result<User, ApiError> {
        self.user_repo.find_by_google_id(google_id).await
    }

    pub async fn find_by_id(&self, id: i32) -> Result<User, ApiError> {
        self.user_repo.find_by_id(id).await
    }

    pub async fn create(&self, user: &User) -> Result<User, ApiError> {
        self.user_repo.create(user).await
    }

    pub async fn update(&self, user: &User) -> Result<User, ApiError> {
        self.user_repo.update(user).await
    }

    pub async fn create_profile(&self, profile: &UserProfile) -> Result<UserProfile, ApiError> {
        self.user_repo.create_profile(profile).await
    }

    pub async fn update_profile(&self, profile: &UserProfile) -> Result<UserProfile, ApiError> {
        self.user_repo.update_profile(profile).await
    }

    pub async fn find_profile_by_user_id(&self, user_id: i32) -> Result<UserProfile, ApiError> {
        self.user_repo.find_profile_by_user_id(user_id).await
    }

    pub async fn find_profile_by_id(&self, id: &str) -> Result<UserProfile, ApiError> {
        self.user_repo.find_profile_by_id(id).await
    }
}

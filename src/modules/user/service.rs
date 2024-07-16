use std::sync::Arc;

use crate::error::ApiError;

use super::{ports::Repository, User, UserProfile, UserSetting};

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

    pub async fn update_profile(&self, user: &UserProfile) -> Result<UserProfile, ApiError> {
        self.user_repo.update_profile(user).await
    }

    pub async fn find_profile_by_user_id(&self, user_id: i32) -> Result<UserProfile, ApiError> {
        self.user_repo.find_profile_by_user_id(user_id).await
    }

    pub async fn find_profile_by_id(&self, id: &str) -> Result<UserProfile, ApiError> {
        self.user_repo.find_profile_by_id(id).await
    }

    pub async fn create_settings(&self, setting: &UserSetting) -> Result<UserSetting, ApiError> {
        self.user_repo.create_settings(setting).await
    }

    pub async fn update_settings(&self, setting: &UserSetting) -> Result<UserSetting, ApiError> {
        self.user_repo.update_settings(setting).await
    }

    pub async fn find_settings_by_user_id(&self, user_id: i32) -> Result<UserSetting, ApiError> {
        self.user_repo.find_settings_by_user_id(user_id).await
    }

    pub async fn find_settings_by_profile_id(
        &self,
        profile_id: &str,
    ) -> Result<UserSetting, ApiError> {
        self.user_repo.find_settings_by_profile_id(profile_id).await
    }
}

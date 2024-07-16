use crate::error::ApiError;
use crate::modules::user::ports::Repository;
use crate::modules::user::{User, UserProfile, UserSetting};
use crate::utils::database::PostgresRepository;
use async_trait::async_trait;
use sqlx::Error as SqlxError;

#[async_trait]
impl Repository for PostgresRepository {
    async fn find_by_google_id(&self, google_id: &str) -> Result<User, ApiError> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE google_id = $1")
            .bind(google_id)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| match e {
                SqlxError::RowNotFound => {
                    ApiError::NotFound(format!("User with Google ID {} not found", google_id))
                }
                _ => ApiError::DatabaseError(e),
            })
    }

    async fn find_by_id(&self, id: i32) -> Result<User, ApiError> {
        sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(id)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| match e {
                SqlxError::RowNotFound => {
                    ApiError::NotFound(format!("User with ID {} not found", id))
                }
                _ => ApiError::DatabaseError(e),
            })
    }

    async fn create(&self, user: &User) -> Result<User, ApiError> {
        sqlx::query_as::<_, User>(
            "INSERT INTO users (google_id, email) VALUES ($1, $2) RETURNING *",
        )
        .bind(&user.google_id)
        .bind(&user.email)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| match e {
            SqlxError::Database(db_error) if db_error.constraint().is_some() => {
                ApiError::Conflict("User with this Google ID or email already exists".to_string())
            }
            _ => ApiError::DatabaseError(e),
        })
    }

    async fn update(&self, user: &User) -> Result<User, ApiError> {
        sqlx::query_as::<_, User>("UPDATE users SET email = $1 WHERE id = $2 RETURNING *")
            .bind(&user.email)
            .bind(user.id)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| match e {
                SqlxError::RowNotFound => {
                    ApiError::NotFound(format!("User with ID {} not found", user.id))
                }
                _ => ApiError::DatabaseError(e),
            })
    }

    async fn create_profile(&self, profile: &UserProfile) -> Result<UserProfile, ApiError> {
        sqlx::query_as::<_, UserProfile>(
            "INSERT INTO user_profiles (id, user_id, email, name, bio, phone_number, location, profile_picture_url, theme, custom_url, job_title, facebook_url, twitter_url, instagram_url, linkedin_url) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15) RETURNING *",
        )
        .bind(&profile.id)
        .bind(profile.user_id)
        .bind(&profile.email)
        .bind(&profile.name)
        .bind(&profile.bio)
        .bind(&profile.phone_number)
        .bind(&profile.location)
        .bind(&profile.profile_picture_url)
        .bind(&profile.theme)
        .bind(&profile.custom_url)
        .bind(&profile.job_title)
        .bind(&profile.facebook_url)
        .bind(&profile.twitter_url)
        .bind(&profile.instagram_url)
        .bind(&profile.linkedin_url)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| match e {
            SqlxError::Database(db_error) if db_error.constraint().is_some() => {
                ApiError::Conflict("User profile with this custom URL already exists".to_string())
            }
            _ => ApiError::DatabaseError(e),
        })
    }

    async fn update_profile(&self, profile: &UserProfile) -> Result<UserProfile, ApiError> {
        sqlx::query_as::<_, UserProfile>(
            "UPDATE user_profiles SET 
             email = $1, name = $2, bio = $3, phone_number = $4, location = $5, 
             profile_picture_url = $6, theme = $7, custom_url = $8,
             job_title = $9, facebook_url = $10, twitter_url = $11,
             instagram_url = $12, linkedin_url = $13
             WHERE user_id = $14 RETURNING *",
        )
        .bind(&profile.email)
        .bind(&profile.name)
        .bind(&profile.bio)
        .bind(&profile.phone_number)
        .bind(&profile.location)
        .bind(&profile.profile_picture_url)
        .bind(&profile.theme)
        .bind(&profile.custom_url)
        .bind(&profile.job_title)
        .bind(&profile.facebook_url)
        .bind(&profile.twitter_url)
        .bind(&profile.instagram_url)
        .bind(&profile.linkedin_url)
        .bind(profile.user_id)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| match e {
            SqlxError::RowNotFound => ApiError::NotFound(format!(
                "User profile for user ID {} not found",
                profile.user_id
            )),
            _ => ApiError::DatabaseError(e),
        })
    }

    async fn find_profile_by_user_id(&self, user_id: i32) -> Result<UserProfile, ApiError> {
        sqlx::query_as::<_, UserProfile>("SELECT * FROM user_profiles WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| match e {
                SqlxError::RowNotFound => {
                    ApiError::NotFound(format!("User profile for user ID {} not found", user_id))
                }
                _ => ApiError::DatabaseError(e),
            })
    }

    async fn find_profile_by_id(&self, id: &str) -> Result<UserProfile, ApiError> {
        sqlx::query_as::<_, UserProfile>("SELECT * FROM user_profiles WHERE id = $1")
            .bind(id)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| match e {
                SqlxError::RowNotFound => {
                    ApiError::NotFound(format!("User profile with ID {} not found", id))
                }
                _ => ApiError::DatabaseError(e),
            })
    }

    async fn create_settings(&self, setting: &UserSetting) -> Result<UserSetting, ApiError> {
        sqlx::query_as::<_, UserSetting>(
            "INSERT INTO user_settings (user_id, exchange_contacts, save_contact, call_me, email_me, social_media, template) 
             VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
        )
        .bind(setting.user_id)
        .bind(setting.exchange_contacts)
        .bind(setting.save_contact)
        .bind(setting.call_me)
        .bind(setting.email_me)
        .bind(setting.social_media)
        .bind(&setting.template)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| match e {
            SqlxError::Database(db_error) if db_error.constraint().is_some() => {
                ApiError::Conflict("User settings for this user already exist".to_string())
            }
            _ => ApiError::DatabaseError(e),
        })
    }

    async fn update_settings(&self, setting: &UserSetting) -> Result<UserSetting, ApiError> {
        sqlx::query_as::<_, UserSetting>(
            "UPDATE user_settings SET 
             exchange_contacts = $1, save_contact = $2, call_me = $3, email_me = $4, social_media = $5, template = $6
             WHERE user_id = $7 RETURNING *",
        )
        .bind(setting.exchange_contacts)
        .bind(setting.save_contact)
        .bind(setting.call_me)
        .bind(setting.email_me)
        .bind(setting.social_media)
        .bind(&setting.template)
        .bind(setting.user_id)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| match e {
            SqlxError::RowNotFound => ApiError::NotFound(format!(
                "User settings for user ID {} not found",
                setting.user_id
            )),
            _ => ApiError::DatabaseError(e),
        })
    }

    async fn find_settings_by_user_id(&self, user_id: i32) -> Result<UserSetting, ApiError> {
        sqlx::query_as::<_, UserSetting>("SELECT * FROM user_settings WHERE user_id = $1")
            .bind(user_id)
            .fetch_one(&*self.pool)
            .await
            .map_err(|e| match e {
                SqlxError::RowNotFound => {
                    ApiError::NotFound(format!("User settings for user ID {} not found", user_id))
                }
                _ => ApiError::DatabaseError(e),
            })
    }

    async fn find_settings_by_profile_id(&self, profile_id: &str) -> Result<UserSetting, ApiError> {
        sqlx::query_as::<_, UserSetting>(
            "SELECT us.* FROM user_settings us
         INNER JOIN user_profiles up ON us.user_id = up.user_id
         WHERE up.id = $1",
        )
        .bind(profile_id)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| match e {
            SqlxError::RowNotFound => ApiError::NotFound(format!(
                "User settings for profile ID {} not found",
                profile_id
            )),
            _ => ApiError::DatabaseError(e),
        })
    }
}

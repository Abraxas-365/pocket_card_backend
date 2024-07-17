use crate::error::ApiError;
use crate::modules::user::ports::Repository;
use crate::modules::user::{User, UserProfile};
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
            "INSERT INTO user_profiles (id, user_id, email, name, bio, phone_number, location, 
             profile_picture_url, theme, template, custom_url, job_title, facebook_url, twitter_url, 
             instagram_url, linkedin_url, exchange_contacts, save_contact, call_me, email_me, social_media) 
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19, $20, $21) 
             RETURNING *",
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
        .bind(&profile.template)
        .bind(&profile.custom_url)
        .bind(&profile.job_title)
        .bind(&profile.facebook_url)
        .bind(&profile.twitter_url)
        .bind(&profile.instagram_url)
        .bind(&profile.linkedin_url)
        .bind(profile.exchange_contacts)
        .bind(profile.save_contact)
        .bind(profile.call_me)
        .bind(profile.email_me)
        .bind(profile.social_media)
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
             profile_picture_url = $6, theme = $7, template = $8, custom_url = $9,
             job_title = $10, facebook_url = $11, twitter_url = $12,
             instagram_url = $13, linkedin_url = $14, exchange_contacts = $15,
             save_contact = $16, call_me = $17, email_me = $18, social_media = $19
             WHERE id = $20 RETURNING *",
        )
        .bind(&profile.email)
        .bind(&profile.name)
        .bind(&profile.bio)
        .bind(&profile.phone_number)
        .bind(&profile.location)
        .bind(&profile.profile_picture_url)
        .bind(&profile.theme)
        .bind(&profile.template)
        .bind(&profile.custom_url)
        .bind(&profile.job_title)
        .bind(&profile.facebook_url)
        .bind(&profile.twitter_url)
        .bind(&profile.instagram_url)
        .bind(&profile.linkedin_url)
        .bind(profile.exchange_contacts)
        .bind(profile.save_contact)
        .bind(profile.call_me)
        .bind(profile.email_me)
        .bind(profile.social_media)
        .bind(&profile.id)
        .fetch_one(&*self.pool)
        .await
        .map_err(|e| match e {
            SqlxError::RowNotFound => {
                ApiError::NotFound(format!("User profile with ID {} not found", profile.id))
            }
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
}

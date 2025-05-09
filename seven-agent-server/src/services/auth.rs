use crate::models::user::{User, CreateUserDto, LoginDto, TokenResponse};
use crate::errors::AppError;
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{DateTime, Utc, Duration};
use uuid::Uuid;
use sqlx::{PgPool, Row};

pub struct AuthService {
    db: PgPool,
    jwt_secret: String,
}

impl AuthService {
    pub fn new(db: PgPool, jwt_secret: String) -> Self {
        Self { db, jwt_secret }
    }

    pub async fn register(&self, dto: CreateUserDto) -> Result<User, AppError> {
        // Check if user already exists
        let existing_user = sqlx::query_as::<_, User>(
            r#"SELECT id, username, email, phone, password_hash, name, 
               created_at, updated_at
               FROM users WHERE username = $1"#
        )
        .bind(dto.username.clone())
        .fetch_optional(&self.db)
        .await?;

        if existing_user.is_some() {
            return Err(AppError::UserAlreadyExists);
        }

        // Hash password
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(dto.password.as_bytes(), &salt)
            .map_err(|_| AppError::PasswordError)?
            .to_string();

        // Create user
        let record = sqlx::query_as::<_, User>(
            r#"INSERT INTO users (username, email, phone, password_hash, name, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
            RETURNING id, username, email, phone, password_hash, name, created_at, updated_at"#
        )
        .bind(dto.username)
        .bind(dto.email)
        .bind(dto.phone)
        .bind(password_hash)
        .bind(dto.name)
        .fetch_one(&self.db)
        .await?;

        Ok(record)
    }

    pub async fn login(&self, dto: LoginDto) -> Result<TokenResponse, AppError> {
        // Find user
        let record = sqlx::query_as::<_, User>(
            r#"SELECT id, username, email, phone, password_hash, name, created_at, updated_at
               FROM users WHERE username = $1"#
        )
        .bind(dto.username)
        .fetch_optional(&self.db)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

        // Verify password
        let parsed_hash = PasswordHash::new(&record.password_hash)
            .map_err(|_| AppError::InvalidCredentials)?;
        Argon2::default()
            .verify_password(dto.password.as_bytes(), &parsed_hash)
            .map_err(|_| AppError::InvalidCredentials)?;

        // Generate token
        let token = Uuid::new_v4().to_string();
        Ok(TokenResponse { token })
    }

    pub async fn forgot_password(&self, username: String) -> Result<(), AppError> {
        // Find user
        let record = sqlx::query_as::<_, User>(
            r#"SELECT id, username, email, phone, password_hash, name, created_at, updated_at
               FROM users WHERE username = $1"#
        )
        .bind(username)
        .fetch_optional(&self.db)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

        // Generate reset token
        let token = Uuid::new_v4().to_string();
        let expires_at = Utc::now() + chrono::Duration::hours(24);

        // Store token
        sqlx::query(
            r#"
            INSERT INTO password_reset_tokens (user_id, token, expires_at, created_at)
            VALUES ($1, $2, $3, CURRENT_TIMESTAMP)
            "#
        )
        .bind(record.id)
        .bind(&token)
        .bind(expires_at)
        .execute(&self.db)
        .await?;

        // TODO: Send email with reset link
        println!("Password reset link: http://localhost:8000/reset-password?token={}", token);

        Ok(())
    }

    pub async fn reset_password(&self, token: String, new_password: String) -> Result<(), AppError> {
        // Find token
        let token_record = sqlx::query(
            r#"
            SELECT user_id FROM password_reset_tokens
            WHERE token = $1 AND expires_at > CURRENT_TIMESTAMP
            "#
        )
        .bind(&token)
        .fetch_optional(&self.db)
        .await?
        .ok_or(AppError::InvalidToken)?;

        let user_id: i32 = token_record.get("user_id");

        // Hash new password
        let salt = SaltString::generate(&mut OsRng);
        let argon2 = Argon2::default();
        let password_hash = argon2
            .hash_password(new_password.as_bytes(), &salt)
            .map_err(|_| AppError::PasswordError)?
            .to_string();

        // Update password
        sqlx::query(
            r#"
            UPDATE users
            SET password_hash = $1, updated_at = CURRENT_TIMESTAMP
            WHERE id = $2
            "#
        )
        .bind(password_hash)
        .bind(user_id)
        .execute(&self.db)
        .await?;

        // Delete used token
        sqlx::query(
            "DELETE FROM password_reset_tokens WHERE token = $1"
        )
        .bind(token)
        .execute(&self.db)
        .await?;

        Ok(())
    }
}
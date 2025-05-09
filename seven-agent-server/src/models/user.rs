use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use utoipa::ToSchema;
use sqlx::{FromRow, Row, postgres::PgRow};

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct User {
    pub id: i32,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl<'r> FromRow<'r, PgRow> for User {
    fn from_row(row: &'r PgRow) -> sqlx::Result<Self> {
        Ok(Self {
            id: row.get("id"),
            username: row.get("username"),
            password_hash: row.get("password_hash"),
            name: row.get("name"),
            email: row.get("email"),
            phone: row.get("phone"),
            created_at: row.get("created_at"),
            updated_at: row.get("updated_at"),
        })
    }
}

#[derive(Debug, Deserialize, ToSchema, Clone)]
pub struct CreateUserDto {
    pub username: String,
    pub password: String,
    pub name: String,
    pub email: Option<String>,
    pub phone: Option<String>,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct LoginDto {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ForgotPasswordDto {
    pub username: String,
}

#[derive(Debug, Deserialize, ToSchema)]
pub struct ResetPasswordDto {
    pub token: String,
    pub new_password: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TokenResponse {
    pub token: String,
}
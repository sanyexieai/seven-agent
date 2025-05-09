use rocket::serde::json::Json;
use rocket::{post, routes};
use crate::models::user::{CreateUserDto, LoginDto, ForgotPasswordDto, ResetPasswordDto, TokenResponse};
use crate::services::auth::AuthService;
use crate::errors::AppError;
use utoipa::OpenApi;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![register, login, forgot_password, reset_password]
}

/// Register a new user
#[utoipa::path(
    post,
    path = "/api/register",
    request_body = CreateUserDto,
    responses(
        (status = 200, description = "User registered successfully", body = TokenResponse),
        (status = 400, description = "Invalid input"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/register", format = "json", data = "<dto>")]
pub async fn register(
    dto: Json<CreateUserDto>,
    auth_service: &rocket::State<AuthService>,
) -> Result<Json<TokenResponse>, AppError> {
    let dto = dto.into_inner();
    let user = auth_service.register(dto.clone()).await?;
    let token = auth_service.login(LoginDto {
        username: user.username.clone(),
        password: dto.password,
    }).await?;
    Ok(Json(token))
}

/// Login user
#[utoipa::path(
    post,
    path = "/api/login",
    request_body = LoginDto,
    responses(
        (status = 200, description = "Login successful", body = TokenResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/login", format = "json", data = "<dto>")]
pub async fn login(
    dto: Json<LoginDto>,
    auth_service: &rocket::State<AuthService>,
) -> Result<Json<TokenResponse>, AppError> {
    let token = auth_service.login(dto.into_inner()).await?;
    Ok(Json(token))
}

/// Request password reset
#[utoipa::path(
    post,
    path = "/api/forgot-password",
    request_body = ForgotPasswordDto,
    responses(
        (status = 200, description = "Password reset email sent"),
        (status = 404, description = "User not found"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/forgot-password", format = "json", data = "<dto>")]
pub async fn forgot_password(
    dto: Json<ForgotPasswordDto>,
    auth_service: &rocket::State<AuthService>,
) -> Result<Json<()>, AppError> {
    auth_service.forgot_password(dto.into_inner().username).await?;
    Ok(Json(()))
}

/// Reset password
#[utoipa::path(
    post,
    path = "/api/reset-password",
    request_body = ResetPasswordDto,
    responses(
        (status = 200, description = "Password reset successful"),
        (status = 400, description = "Invalid token"),
        (status = 500, description = "Internal server error")
    )
)]
#[post("/reset-password", format = "json", data = "<dto>")]
pub async fn reset_password(
    dto: Json<ResetPasswordDto>,
    auth_service: &rocket::State<AuthService>,
) -> Result<Json<()>, AppError> {
    let dto = dto.into_inner();
    auth_service.reset_password(dto.token, dto.new_password).await?;
    Ok(Json(()))
}
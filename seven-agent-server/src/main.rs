#[macro_use] extern crate rocket;

mod config;
mod models;
mod routes;
mod services;
mod errors;

use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;
use crate::config::Config;
use crate::services::auth::AuthService;
use sqlx::postgres::PgPoolOptions;
use sqlx::migrate::MigrateDatabase;
use sqlx::Postgres;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::register,
        routes::login,
        routes::forgot_password,
        routes::reset_password
    ),
    components(
        schemas(
            models::user::User,
            models::user::CreateUserDto,
            models::user::LoginDto,
            models::user::ForgotPasswordDto,
            models::user::ResetPasswordDto,
            models::user::TokenResponse
        )
    ),
    tags(
        (name = "auth", description = "Authentication endpoints.")
    )
)]
struct ApiDoc;

#[launch]
async fn rocket() -> _ {
    let config = Config::from_env();
    
    println!("Checking database...");
    if !Postgres::database_exists(&config.database_url).await.unwrap_or(false) {
        println!("Database does not exist, creating...");
        Postgres::create_database(&config.database_url).await.expect("Failed to create database");
    }

    println!("Connecting to database...");
    // Initialize database connection
    let db = PgPoolOptions::new()
        .max_connections(5)
        .connect(&config.database_url)
        .await
        .expect("Failed to connect to database");
    println!("Database connection established.");

    let auth_service = AuthService::new(db, config.jwt_secret);
    let api_doc = ApiDoc::openapi();

    println!("Starting Rocket server...");
    rocket::build()
        .manage(auth_service)
        .mount("/api", routes::get_routes())
        .mount("/", SwaggerUi::new("/swagger-ui/<_..>").url("/api-docs/openapi.json", api_doc))
}

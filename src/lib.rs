use anyhow::Result;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};
use std::net::SocketAddr;
use url::Url;

// اینها رو `pub` می‌کنیم تا از بیرون ماژول (مثل main.rs و تست‌ها) قابل دسترسی باشن
pub type DatabasePool = SqlitePool;

#[derive(Deserialize)]
pub struct ShortenRequest {
    pub url: String,
}

#[derive(Serialize)]
pub struct ShortenResponse {
    pub short_url: String,
}

#[derive(sqlx::FromRow)]
struct UrlRecord {
    original_url: String,
}

pub fn generate_short_code() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}

pub async fn shorten(
    State(pool): State<DatabasePool>,
    Json(payload): Json<ShortenRequest>,
) -> Result<Json<ShortenResponse>, StatusCode> {
    let short_code = generate_short_code();

    sqlx::query("INSERT INTO urls (short_code, original_url) VALUES (?, ?)")
        .bind(&short_code)
        .bind(&payload.url)
        .execute(&pool)
        .await
        .map_err(|e| {
            eprintln!("Failed to insert into database: {}", e);
            StatusCode::INTERNAL_SERVER_ERROR
        })?;

    // TODO: این آدرس باید قابل تنظیم باشه، فعلاً برای تست خوبه
    let base_url = "http://localhost:3000/";
    let full_short_url = Url::parse(base_url)
        .unwrap()
        .join(&short_code)
        .unwrap()
        .to_string();

    let response = ShortenResponse {
        short_url: full_short_url,
    };

    Ok(Json(response))
}

pub async fn redirect(
    State(pool): State<DatabasePool>,
    Path(short_code): Path<String>,
) -> impl IntoResponse {
    let result = sqlx::query_as::<_, UrlRecord>("SELECT original_url FROM urls WHERE short_code = ?")
        .bind(&short_code)
        .fetch_one(&pool)
        .await;

    match result {
        Ok(record) => Redirect::permanent(&record.original_url).into_response(),
        Err(_) => (StatusCode::NOT_FOUND, "URL Not Found").into_response(),
    }
}

// تابعی برای ساختن روتر که در تست‌ها هم ازش استفاده می‌کنیم
pub fn create_app(pool: DatabasePool) -> Router {
    Router::new()
        .route("/shorten", post(shorten))
        .route("/:id", get(redirect))
        .with_state(pool)
}

// تابعی برای راه‌اندازی دیتابیس
pub async fn setup_database(db_url: &str) -> Result<DatabasePool> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS urls (
            id INTEGER PRIMARY KEY,
            short_code TEXT NOT NULL UNIQUE,
            original_url TEXT NOT NULL
        );",
    )
    .execute(&pool)
    .await?;

    Ok(pool)
}

// تست واحد رو هم به اینجا منتقل می‌کنیم
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_short_code_has_correct_length() {
        let code = generate_short_code();
        assert_eq!(code.len(), 6);
    }
}
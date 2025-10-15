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
use url::Url;

type DatabasePool = SqlitePool;

#[derive(Deserialize)]
pub struct ShortenRequest {
    url: String,
}

#[derive(Serialize)]
pub struct ShortenResponse {
    short_url: String,
}

// یک struct جدید برای نگهداری رکوردی که از دیتابیس می‌خونیم
#[derive(sqlx::FromRow)]
struct UrlRecord {
    original_url: String,
}

fn generate_short_code() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}

async fn shorten(
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

// --- تابع redirect کامل شده ---
async fn redirect(
    State(pool): State<DatabasePool>,
    Path(short_code): Path<String>, // کد کوتاه رو از مسیر URL استخراج می‌کنه
) -> impl IntoResponse {
    // با استفاده از `query_as!` یک کوئری SELECT اجرا می‌کنیم
    // و نتیجه رو به `UrlRecord` تبدیل می‌کنیم.
    let result = sqlx::query_as::<_, UrlRecord>("SELECT original_url FROM urls WHERE short_code = ?")
        .bind(&short_code)
        .fetch_one(&pool) // .fetch_one() سعی می‌کنه دقیقاً یک رکورد پیدا کنه
        .await;

    match result {
        // اگر یک رکورد پیدا شد
        Ok(record) => {
            println!("Redirecting {} to {}", short_code, record.original_url);
            // کاربر رو به URL اصلی هدایت می‌کنیم
            Redirect::permanent(&record.original_url).into_response()
        }
        // اگر رکوردی پیدا نشد
        Err(_) => {
            // خطای 404 Not Found برمی‌گردونیم
            (StatusCode::NOT_FOUND, "URL Not Found").into_response()
        }
    }
}


#[tokio::main]
async fn main() -> Result<()> {
    let db_url = "sqlite://urls.db?mode=rwc";
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(db_url)
        .await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS urls (
            id INTEGER PRIMARY KEY,
            short_code TEXT NOT NULL UNIQUE,
            original_url TEXT NOT NULL
        );"
    )
    .execute(&pool)
    .await?;

    println!("پایگاه داده با موفقیت راه‌اندازی شد و جدول 'urls' آماده است.");

    let app = Router::new()
        .route("/shorten", post(shorten))
        .route("/:id", get(redirect))
        .with_state(pool);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("سرور در آدرس http://{} در حال اجراست", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
    
    Ok(())
}

use axum::Router;
use reqwest::{Client, StatusCode};
use sqlx::SqlitePool;
use tokio::net::TcpListener;
// نام کتابخانه شما (پروژه) و توابع عمومی آن
use url_shortener::{create_app, setup_database, DatabasePool, ShortenRequest, ShortenResponse};

/// تابع کمکی برای راه‌اندازی سرور تست
/// این تابع یک دیتابیس در حافظه (memory) می‌سازه که بعد از تست پاک می‌شه
/// و سرور رو روی یک پورت رندوم اجرا می‌کنه
async fn spawn_app() -> (String, DatabasePool) {
    // یک دیتابیس در حافظه برای تست می‌سازیم
    let pool = setup_database("sqlite::memory:")
        .await
        .expect("Failed to create test database.");

    let app = create_app(pool.clone());

    // سرور رو روی یک پورت رندوم (پورت 0) اجرا می‌کنیم
    let listener = TcpListener::bind("127.0.0.1:0")
        .await
        .expect("Failed to bind random port");
    let port = listener.local_addr().unwrap().port();
    let addr = format!("http://127.0.0.1:{}", port);

    // سرور رو در یک ترد جداگانه در پس‌زمینه اجرا می‌کنیم
    tokio::spawn(async move {
        axum::Server::from_tcp(listener)
            .unwrap()
            .serve(app.into_make_service())
            .await
            .unwrap();
    });

    (addr, pool)
}

// این تست اصلی ماست
#[tokio::test]
async fn test_shorten_and_redirect_workflow() {
    // --- آماده‌سازی (Arrange) ---
    // سرور تست رو اجرا می‌کنیم و آدرس و دیتابیس اون رو می‌گیریم
    let (app_address, db_pool) = spawn_app().await;
    let client = Client::new();
    let original_url = "https://www.google.com/very/long/path";

    // --- اجرا (Act) ۱: تست POST /shorten ---
    let response = client
        .post(format!("{}/shorten", app_address))
        .json(&ShortenRequest {
            url: original_url.to_string(),
        })
        .send()
        .await
        .expect("Failed to execute /shorten request");

    // --- اعتبارسنجی (Assert) ۱ ---
    assert_eq!(response.status(), StatusCode::OK); // چک می‌کنیم که درخواست موفق بوده
    let response_json: ShortenResponse = response
        .json()
        .await
        .expect("Failed to parse response JSON");

    let short_code = response_json.short_url.split('/').last().unwrap();

    // چک می‌کنیم که آیا واقعاً در دیتابیس ذخیره شده؟
    let db_record: (String,) =
        sqlx::query_as("SELECT original_url FROM urls WHERE short_code = ?")
            .bind(short_code)
            .fetch_one(&db_pool)
            .await
            .expect("Failed to find record in test DB");
    assert_eq!(db_record.0, original_url); // آیا URL ذخیره شده همونه؟

    // --- اجرا (Act) ۲: تست GET /:id (Redirect) ---
    // یک کلاینت جدید می‌سازیم که Redirect رو دنبال نکنه
    let redirect_client = Client::builder()
        .redirect(reqwest::redirect::Policy::none())
        .build()
        .unwrap();

    let redirect_response = redirect_client
        .get(format!("{}/{}", app_address, short_code))
        .send()
        .await
        .expect("Failed to execute redirect request");

    // --- اعتبارسنجی (Assert) ۲ ---
    // چک می‌کنیم که آیا سرور پاسخ "هدایت دائمی" (308) داده؟
    assert_eq!(redirect_response.status(), StatusCode::PERMANENT_REDIRECT);
    // چک می‌کنیم که آیا هدر 'Location' به آدرس اصلی اشاره داره؟
    let location_header = redirect_response
        .headers()
        .get("Location")
        .expect("No 'Location' header")
        .to_str()
        .unwrap();
    assert_eq!(location_header, original_url);
}
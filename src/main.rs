use anyhow::Result;
// نام پروژه شما (که در Cargo.toml هست) به عنوان کتابخانه در دسترس قرار می‌گیره
use url_shortener::{create_app, setup_database};

#[tokio::main]
async fn main() -> Result<()> {
    // دیتابیس تولیدی (production) رو راه‌اندازی می‌کنیم
    let db_url = "sqlite://urls.db?mode=rwc";
    let pool = setup_database(db_url).await?;
    println!("پایگاه داده با موفقیت راه‌اندازی شد و جدول 'urls' آماده است.");

    // اپلیکیشن رو می‌سازیم
    let app = create_app(pool);

    // سرور رو اجرا می‌کنیم
    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("سرور در آدرس http://{} در حال اجراست", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}

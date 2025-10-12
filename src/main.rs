use axum::{routing::get, Router};

async fn helloworld() -> &'static str {
    "Hello, world!"}
#[tokio::main]
async fn main(){
    let app = Router::new().route("/", get(helloworld));

    let adder = std::net::SocketAddr::from(([127,0,0,1], 3000));
    println!("Listening on {}", adder);

    axum::Server::bind(&adder)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
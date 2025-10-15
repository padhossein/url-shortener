use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{IntoResponse, Redirect},
    routing::{get, post},
    Json, Router,
};
use rand::{distributions::Alphanumeric, Rng};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use url::Url;

type Database = Arc<Mutex<HashMap<String, String>>>;

#[derive(Deserialize)]
pub struct ShortenRequest {
    url: String,
}

#[derive(Serialize)]
pub struct ShortenResponse {
    short_url: String,
}

fn generate_short_code() -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(6)
        .map(char::from)
        .collect()
}

async fn shorten(
    State(db): State<Database>,
    Json(payload): Json<ShortenRequest>,
) -> Json<ShortenResponse> {
    let short_code = generate_short_code();

    db.lock()
        .unwrap()
        .insert(short_code.clone(), payload.url);
    
    let base_url = "http://localhost:3000/"; 
    let full_short_url = Url::parse(base_url)
        .unwrap()
        .join(&short_code)
        .unwrap()
        .to_string();

    let response = ShortenResponse {
        short_url: full_short_url,
    };

    Json(response)
}

async fn redirect(
    Path(short_code): Path<String>,
    State(db): State<Database>,
) -> impl IntoResponse {
    let db_locked = db.lock().unwrap();

    if let Some(long_url) = db_locked.get(&short_code) {
        println!("Redirecting {} to {}", short_code, long_url);
        Redirect::permanent(long_url).into_response()
    } else {
        (StatusCode::NOT_FOUND, "URL Not Found").into_response()
    }
}

#[tokio::main]
async fn main() {
    let db = Database::new(Mutex::new(HashMap::new()));

    
    let app = Router::new()
        .route("/shorten", post(shorten))
        .route("/:id", get(redirect)) 
        .with_state(db);

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("http://{} server is running ", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

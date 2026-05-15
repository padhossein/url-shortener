# URL Shortener

A small Rust web service that turns long URLs into short redirect links. It is built with Axum, Tokio, SQLx, and SQLite, so it is a good practice project for async backend development in Rust.

## Features

- Create short links with a JSON API
- Redirect short codes to the original URL
- Store URL mappings in a local SQLite database
- Use async request handling with Tokio and Axum
- Keep database access typed through SQLx

## Tech Stack

- Rust
- Axum
- Tokio
- SQLx
- SQLite
- Serde

## Getting Started

Clone the repository:

```bash
git clone https://github.com/padhossein/url-shortener.git
cd url-shortener
```

Build the project:

```bash
cargo build
```

Run the server:

```bash
cargo run
```

The server starts at:

```text
http://127.0.0.1:3000
```

The SQLite database file, `urls.db`, is created automatically on first run.

## API Usage

Create a short URL:

```bash
curl -X POST http://127.0.0.1:3000/shorten \
  -H "Content-Type: application/json" \
  -d '{"url":"https://www.rust-lang.org/"}'
```

Example response:

```json
{
  "short_url": "http://localhost:3000/aB1cD2"
}
```

Open the short URL in a browser:

```text
http://localhost:3000/aB1cD2
```

If the code exists, the service redirects to the original URL. If the code is unknown, the service returns `404 Not Found`.

## Development Notes

Useful checks while working on the project:

```bash
cargo fmt
cargo clippy
cargo test
```

## Next Improvements

- Add request validation errors with clearer JSON responses
- Add integration tests for redirects and missing codes
- Add configurable host, port, and database path
- Add Docker support for easier deployment

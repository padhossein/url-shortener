# Rust URL Shortener Service

A simple yet powerful web service built with Rust and Axum that shortens long URLs. This application allows you to create short, manageable links that redirect to their original long-form destinations.

This project was developed as a practical exercise to master key backend development concepts in Rust, including asynchronous programming, database interaction, and building robust APIs.

## ✨ Features

* **Modern API:** Built with the fast and ergonomic **Axum** web framework.
* **Persistent Storage:** Uses **SQLite** for durable, file-based storage of URL mappings.
* **Safe Database Access:** Leverages **SQLx** for compile-time checked, asynchronous, and safe SQL queries.
* **Asynchronous-First:** Fully asynchronous from the web layer to the database, powered by **Tokio**.
* **Robust Error Handling:** Utilizes `anyhow` for clean and unified error management.

## 🛠️ Tech Stack

* **Language:** [Rust](https://www.rust-lang.org/)
* **Web Framework:** [Axum](https://github.com/tokio-rs/axum)
* **Async Runtime:** [Tokio](https://tokio.rs/)
* **Database:** [SQLite](https://www.sqlite.org/index.html)
* **Database Toolkit:** [SQLx](https://github.com/launchbadge/sqlx)
* **Serialization/Deserialization:** [Serde](https://serde.rs/)
* **Random Code Generation:** [Rand](https://crates.io/crates/rand)

## 🚀 Getting Started

Follow these instructions to set up and run the project locally.

### Prerequisites

* You must have the [Rust toolchain](https://www.rust-

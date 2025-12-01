# Inventory Application CLI

Command-line inventory application built in Rust using Actix Web and SQLx (SQLite). This project provides a small HTTP server exposing authentication endpoints and is intended as the backend for an inventory management CLI or web client.

**Key features**
- HTTP API written with `actix-web`.
- Database access via `sqlx` (SQLite feature enabled).
- JSON (de)serialization with `serde`.

**Repository layout**
- `src/` — application source. `src/main.rs` boots the Actix server and registers controllers.
- `src/controllers/` — request handlers (e.g. `auth.rs`).
- `Cargo.toml` — Rust package manifest and dependencies.

Prerequisites
- Rust toolchain (stable). Install via https://rustup.rs
- `cargo` (comes with Rust toolchain)
- Optional: `sqlx-cli` if you plan to use SQLx offline features or migrations


By default the server binds to `127.0.0.1:8080` (see `src/main.rs`).

API Endpoints (examples)
- POST `/auth/signup` — register a new user (example handler present in `src/controllers/auth.rs`).
- POST `/auth/signin` — sign in an existing user.

Database
- `sqlx` is configured for SQLite in `Cargo.toml`. If you add migrations or use `sqlx-cli`, follow the `sqlx` docs for creating/updating the database schema.

Contributing
- Fork the repo, create a feature branch, and open a pull request. Keep changes small and focused.

License
- See the `LICENSE` file in the repository root.

Contact
- Open an issue or PR in this repository for questions or feature requests.


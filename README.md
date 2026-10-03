# RestfulApi

[![License: AGPL v3](https://img.shields.io/badge/License-AGPL_v3-blue.svg)](LICENSE)
[![Platform: Windows](https://img.shields.io/badge/Platform-Windows-0078D6.svg?logo=windows&logoColor=white)](#)

A fully functional REST API server built in Rust using the **Actix-Web** framework. It manages an in-memory `User` store protected by a `Mutex<HashMap>` and exposes full CRUD operations across five HTTP endpoints.

---

## Features

- **Actix-Web Server**: Runs on `localhost:8080` using the `#[actix_web::main]` async runtime macro.
- **Mutex-Protected State**: Shared `AppState` wraps a `Mutex<HashMap<u32, User>>`, ensuring safe concurrent access across all handler threads.
- **Full CRUD**: GET, POST, PUT, and DELETE endpoints manage `User { id: u32, name: String, status: String }` structs, serialized/deserialized by `serde_json`.
- **Conflict & 404 Handling**: `create_user` returns `409 Conflict` if the ID already exists; `delete_user` returns `404 Not Found` if no matching entry is present.

---

## Quick Start

1. Clone or download the repository.
2. Install the Rust toolchain via `rustup`.
3. Run `cargo run` to launch the server on `localhost:8080`.

---

## Configuration Details

No external database or configuration file is required. The server pre-seeds the in-memory `HashMap` with a default user (`id: 1, name: "John Doe", status: "Active"`) on startup for immediate testing.

---

## Usage Guidelines

```sh
# Health check
curl -X GET http://localhost:8080/

# Fetch a user by ID
curl -X GET http://localhost:8080/users/1

# Create a new user
curl -X POST http://localhost:8080/users -H "Content-Type: application/json" -d "{\"id\": 2, \"name\": \"Jane\", \"status\": \"Active\"}"

# Update a user
curl -X PUT http://localhost:8080/update -H "Content-Type: application/json" -d "{\"id\": 1, \"name\": \"Updated\", \"status\": \"Inactive\"}"

# Delete a user
curl -X DELETE http://localhost:8080/del/1
```

---

## Technical Documentation

For developers interested in directory structures, code architecture, or compilation guidelines, please refer to the **[Documentation.md](Documentation.md)** file.

---

## License

This project is licensed under the **GNU Affero General Public License Version 3 (AGPLv3)**. See the LICENSE file for details.

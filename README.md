# RestfulApi: High-Performance Lightweight Rust User API

[![Rust](https://img.shields.io/badge/rust-v1.93+-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org)
[![Actix Web](https://img.shields.io/badge/Actix--Web-v4.13-blue.svg?style=flat-square)](https://actix.rs)
[![License](https://img.shields.io/badge/license-MIT-green.svg?style=flat-square)](LICENSE)

A highly efficient, robust, and lightweight RESTful API for managing user state, built with **Rust** and the **Actix Web** framework. This application is compiled into a standalone, dependency-free binary file (`launcher.exe`), featuring a custom API-themed application icon, allowing for zero-install deployment on Windows.

---

<div align="center">

### **Instant Standalone Download**
Get the pre-compiled, fully self-contained Windows executable containing all dependencies.

[![Download launcher.exe](https://img.shields.io/badge/Download-launcher.exe-007ACC?style=for-the-badge&logo=windows&logoColor=white)](https://github.com/Joao-Tolentino/RestfulApi/releases/latest/download/launcher.exe)

</div>

---

## Key Features

- **Blazing Fast performance**: Leverages Actix Web, one of the fastest asynchronous web frameworks available.
- **Self-Contained Executable**: No runtime dependencies, no DLLs, and no external installations required. Just download and run.
- **In-Memory Thread-Safe Storage**: Implements safe concurrent user state manipulation using Rust's `Mutex` and `HashMap`.
- **Comprehensive CRUD Operations**: Fully-featured user state management.
- **Professional Windows Integration**: Embedded system resource information and custom application icon.

## API Endpoint Reference

The API runs by default on `http://localhost:8080`. Below is the concise list of endpoints:

| Method | Endpoint | Description | Request Payload | Response Code |
| :--- | :--- | :--- | :--- | :--- |
| **GET** | `/` | Verify API health status | *None* | `200 OK` |
| **GET** | `/users/{id}` | Retrieve a user profile by ID | *None* | `200 OK` / `404 Not Found` |
| **POST** | `/users` | Create a new user entry | `User` JSON | `201 Created` / `409 Conflict` |
| **PUT** | `/update` | Update existing user or create if new | `User` JSON | `200 OK` / `201 Created` |
| **DELETE** | `/del/{id}` | Delete a user profile by ID | *None* | `204 No Content` / `404 Not Found` |

### User Schema

All user payload data must conform to the following JSON structure:

```json
{
  "id": 1,
  "name": "John Doe",
  "status": "Active"
}
```

---

## Quick Start Guide

### 1. Running the Executable
Simply download the pre-compiled `launcher.exe` and execute it from your file manager or terminal:

```cmd
launcher.exe
```

Upon launching, the console will output:
```text
Server running in localhost:8080
```

### 2. Testing the Endpoints
You can easily test the REST API endpoints using standard Windows command-line `curl.exe`:

*   **Check API Status:**
    ```cmd
    curl.exe -X GET http://localhost:8080/
    ```
*   **Get User with ID 1:**
    ```cmd
    curl.exe -X GET http://localhost:8080/users/1
    ```
*   **Create New User:**
    ```cmd
    curl.exe -X POST "http://localhost:8080/users" -H "Content-Type: application/json" -d "{\"id\": 2, \"name\": \"Jane Smith\", \"status\": \"Active\"}"
    ```
*   **Update User:**
    ```cmd
    curl.exe -X PUT "http://localhost:8080/update" -H "Content-Type: application/json" -d "{\"id\": 2, \"name\": \"Jane Smith\", \"status\": \"Inactive\"}"
    ```
*   **Delete User:**
    ```cmd
    curl.exe -X DELETE http://localhost:8080/del/2
    ```

---

## Additional Technical Details

For deep technical details about architecture, concurrency management, and building from source, please refer to the [Documentation.md](file:///c:/Users/joaos/Learning%20Project/Rust/RestfulApi/Documentation.md) file.

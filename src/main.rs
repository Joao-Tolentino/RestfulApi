// Imports
use actix_web::{get, post, put, delete, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;

// Define a struct to format the data and the payloads JSON
#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    status: String,
}

// Define the endpoints
// GET endpoint in root "/"
#[get("/")]
async fn test() -> impl Responder {
    // Respond the check endpoint
    HttpResponse::Ok().json(json!({"message": "The Server and API are working!"}))
}

// GET endpoint in "/users/*id*" fetch and user by id
#[get("/users/{id}")]
async fn fetch_user(path: web::Path<u32>) -> impl Responder {
    // Extract ID from payload
    let user_id = path.into_inner();

    // Create a dummy user for the response
    let user = User {
        id: user_id,
        name: "John Doe".to_string(),
        status: "Active".to_string(),
    };

    // Respond with the user as JSON
    HttpResponse::Ok().json(user)
}

// POST endpoint in "/users" to create new entry
#[post("/users")]
async fn create_user(user: web::Json<User>) -> impl Responder {
    // Uses the User struct to deserialize the payload
    let new_user = user.into_inner();

    // Respond with the same data and 201 status Created
    HttpResponse::Created().json(new_user)
}

// PUT endpoint in "/update/id" to fully change the data
#[put("/update/{id}")]
async fn update(path: web::Path<u32>) -> impl Responder {
    // Extract the ID
    let user_id = path.into_inner();

    // Updates the entry
    println!("{}", user_id);

    // Respond with the 200 status OK
    HttpResponse::Ok().finish()
}

// DELETE endpoint in "/del/id" to delete an user
#[delete("/del/{id}")]
async fn delete_user(path: web::Path<u32>) -> impl Responder {
    // Extract the ID
    let user_id = path.into_inner();

    // Deletes the user
    println!("{}", user_id);

    // Responde with 200 status
    HttpResponse::Ok().finish()
}

// The main entry point for the app
#[actix_web::main] // runs async main with Actix Web
async fn main() -> std::io::Result<()> {
    // Print the server is started
    println!("Server running in localhost:8080");
    
    // Start the HTTP server
    HttpServer::new(|| {
        App::new()
            .service(test)
            .service(fetch_user)
            .service(create_user)
            .service(update)
            .service(delete_user)
    })
    // Bind the server to 8080 port
    .bind(("localhost", 8080))?
    // Run server until stopped
    .run()
    .await

    /* Curl for testing
    curl.exe -X GET http://localhost:8080/
    curl.exe -X GET http://localhost:8080/users/1
    curl.exe -X PUT http://localhost:8080/update/1
    curl.exe -X DELETE http://localhost:8080/del/1

    curl.exe -X POST "http://localhost:8080/users" -H "Content-Type: application/json" -d "{\"id\": 1, \"name\": \"Test123\", \"status\": \"Active\"}"
    */
}

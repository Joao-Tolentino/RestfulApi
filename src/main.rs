// Imports
use actix_web::{get, post, put, delete, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::sync::Mutex;

// Define a struct to format the data and the payloads JSON
#[derive(Serialize, Deserialize, Clone)]
struct User {
    id: u32,
    name: String,
    status: String,
}

// Define temporary App State
struct AppState {
    users: Mutex<HashMap<u32, User>>,
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
async fn fetch_user(
    state: web::Data<AppState>,
    path: web::Path<u32>,
) -> impl Responder {
    // Extract ID from payload
    let user_id = path.into_inner();

    // Connect to the current state
    let users = state.users.lock().unwrap();

    // Find user and respond its struct or respond 404 status
    match users.get(&user_id) {
        Some(user) => HttpResponse::Ok().json(user),
        None => HttpResponse::NotFound().finish(),
    }
}

// POST endpoint in "/users" to create new entry
#[post("/users")]
async fn create_user(
    state: web::Data<AppState>,
    user: web::Json<User>,
) -> impl Responder {
    // Check the current state
    let mut users = state.users.lock().unwrap();
    
    // Uses the User struct to deserialize the payload
    let new_user = user.into_inner();

    // Check for user with same id, insert new if doesnt conflict
    if users.contains_key(&new_user.id) {
        return HttpResponse::Conflict().finish(); // 409
    }
    users.insert(new_user.id, new_user.clone());

    // Respond with the same data and 201 status Created
    HttpResponse::Created().json(new_user)
}

// PUT endpoint in "/update/id" to fully change the data
#[put("/update")]
async fn update(
    state: web::Data<AppState>,
    user: web::Json<User>,
) -> impl Responder {
    // Check current state and get the user
    let mut users = state.users.lock().unwrap();
    let updated_user = user.into_inner();

    // Check if user exists
    let existed = users.contains_key(&updated_user.id);

    // Makes the change or simply create a new user entry
    users.insert(updated_user.id, updated_user);

    // Responds with 200 OK if change succesful, or 201 Created if new entry was made 
    if existed {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::Created().finish()
    }
}

// DELETE endpoint in "/del/id" to delete an user
#[delete("/del/{id}")]
async fn delete_user(
    state: web::Data<AppState>,
    path: web::Path<u32>,
) -> impl Responder {
    // Check the current state
    let mut users = state.users.lock().unwrap();

    // Extract the ID
    let user_id = path.into_inner();

    // Checks if the user exists and delete its entry, respond 404 if no user found
    if users.remove(&user_id).is_some() {
        HttpResponse::NoContent().finish() // 204
    } else {
        HttpResponse::NotFound().finish() // 404
    }
}

// The main entry point for the app
#[actix_web::main] // runs async main with Actix Web
async fn main() -> std::io::Result<()> {
    // Print the server is started
    println!("Server running in localhost:8080");

    // Create the HashMap storage
    let mut users = HashMap::new();

    //Insert dummy user in the hash
    users.insert(
        1,
        User {
            id: 1,
            name: "John Doe".to_string(),
            status: "Active".to_string(),
        },
    );

    

    // Initialize the data state
    let state = web::Data::new(AppState {
        users: Mutex::new(users),
    });
    
    // Start the HTTP server
    HttpServer::new(move || {
        App::new()
            .app_data(state.clone())
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
    curl.exe -X DELETE http://localhost:8080/del/1
    
    // Curl with payload
    curl.exe -X POST "http://localhost:8080/users" -H "Content-Type: application/json" -d "{\"id\": 1, \"name\": \"Test123\", \"status\": \"Active\"}"
    curl.exe -X PUT "http://localhost:8080/update" -H "Content-Type: application/json" -d "{\"id\":2,\"name\":\"Test123\",\"status\":\"Active\"}"
    */
}

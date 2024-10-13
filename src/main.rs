mod services;
mod models;

use aws_config::defaults;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use env_logger;
use actix_web::web::Data;
use actix_web::{App, HttpServer};
use crate::services::{UserService, UserServiceImpl, register, login, register_vehicle};
use std::sync::Arc; // Import Arc

#[actix_web::main]
async fn main() -> std::io::Result<()> {

    env_logger::init();
   

    // Load the configuration for AWS SDK
    let config = defaults(BehaviorVersion::latest())
        .test_credentials()
        .endpoint_url("http://localhost:8000")
        .load()
        .await;

    // Create DynamoDB config using the loaded configuration
    let dynamodb_local_config = aws_sdk_dynamodb::config::Builder::from(&config).build();

    // Create a DynamoDB client
    let client = Client::from_conf(dynamodb_local_config);

    let user_service = UserServiceImpl {
        client,
        table_name: String::from("user-info"), // Ensure this matches your table name
    };

    // Start the HTTP server
    HttpServer::new(move || {
        let user_service_arc: Arc<dyn UserService> = Arc::new(user_service.clone());
        let user_service_data: Data<dyn UserService> = Data::from(user_service_arc);
        App::new()
            .app_data(user_service_data) // Register user_service
            .service(register) // Register the register endpoint
            .service(register_vehicle) // Register the vehicle endpoint
            .service(login) // Register the login endpoint
    })
    .bind("127.0.0.1:8081")? // Handle potential bind errors
    .run() // Start the server
    .await // Await for the server to run
}

mod services;
mod models;
mod utils;

use aws_config::defaults;
use aws_config::BehaviorVersion;
use aws_sdk_dynamodb::Client;
use env_logger;
use actix_web::web::Data;
use crate::models::VehicleType;
use actix_web::{App, HttpServer};
use crate::services::{UserService, UserServiceImpl, register, login, register_vehicle, LotService, LotServiceImpl, get_available_slots};
use std::sync::Arc;

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


    let client = Arc::new(Client::from_conf(dynamodb_local_config));

    let user_service = UserServiceImpl {
        client: Arc::clone(&client), // Clone the Arc to share ownership
        table_name: String::from("user-info"),
    };

    let parking_service = LotServiceImpl {
        client: Arc::clone(&client), // Clone the Arc to share ownership
        table_name: String::from("parking-info"), 
    };
    let vehicle_type = VehicleType::SMALL;
        // Fetch available slots by awaiting the future
        match parking_service.find_lots(vehicle_type).await {
            Ok(slots) => println!("Available slots: {:?}", slots),
            Err(e) => eprintln!("Error fetching slots: {:?}", e),
        }


    
    // Start the HTTP server
    HttpServer::new(move || {
        let user_service_arc: Arc<dyn UserService> = Arc::new(user_service.clone());
        let user_service_data: Data<dyn UserService> = Data::from(user_service_arc);

        let parking_service_arc: Arc<dyn LotService> = Arc::new(parking_service.clone());
        let parking_service_data: Data<dyn LotService> = Data::from(parking_service_arc);
        App::new()
            .app_data(user_service_data)
            .app_data(parking_service_data) 
            .service(get_available_slots) // Register the available slots endpoint
            .service(register) // Register the register endpoint
            .service(register_vehicle) // Register the vehicle endpoint
            .service(login) // Register the login endpoint
    })
    .bind("127.0.0.1:8081")? // Handle potential bind errors
    .run() // Start the server
    .await // Await for the server to run
}

use actix_web::{post, web, HttpResponse, Responder};
use crate::models::User;
use crate::models::LoginRequest;

use crate::services::UserService;
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::Serialize;
use log::info;

#[derive(Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[post("/register")]
pub async fn register(
    user: web::Json<User>,
    data: web::Data<dyn UserService>,
) -> impl Responder {
    info!("Received registration request for user: {:?}", user);   
    match data.create_user(user.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            println!("Error: {:?}", e);
            HttpResponse::InternalServerError().body("Error creating user")
        }
    }
}

#[post("/login")]
pub async fn login(
    user: web::Json<LoginRequest>,
    data: web::Data<dyn UserService>,
) -> impl Responder {
    info!("Received login request for user: {:?}", user);   
    let username = user.email.clone();
    match data.get_user(&username).await {
        Ok(Some(db_user)) => {
            if db_user.password == user.password {
                // Generate JWT
                let claims = Claims {
                    sub: username,
                    exp: 10000000000, // Set expiration as needed
                };
                let secret = "some-secret-key";
                let token = encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_ref()))
                    .map_err(|_| HttpResponse::InternalServerError().body("Error generating token"));
                if let Ok(token) = token {
                    // Return only the token as JSON
                    HttpResponse::Ok().json(token)
                } else {
                    HttpResponse::InternalServerError().body("Error generating token")
                }
            } else {
                HttpResponse::Unauthorized().body("Invalid credentials")
            }
        }
        Ok(None) => HttpResponse::Unauthorized().body("User not found"),
        Err(e) => {
            println!("Error: {:?}", e);
            HttpResponse::InternalServerError().body("Error logging in")
        }
    }
}
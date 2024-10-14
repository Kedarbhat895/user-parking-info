use actix_web::{post, web, HttpRequest, HttpResponse, Responder};
use crate::models::{User, LoginRequest, VehicleRegistrationInfo};
use crate::services::UserService;
use crate::utils::auth::{Claims, extract_token, decode_token, SECRET};
use log::{info, error};

#[post("/register")]
pub async fn register(
    user: web::Json<User>,
    data: web::Data<dyn UserService>,
) -> impl Responder {
    info!("Received registration request for user: {:?}", user);
    match data.update_user(user.into_inner()).await {
        Ok(_) => HttpResponse::Created().finish(),
        Err(e) => {
            error!("Error: {:?}", e);
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
                let claims = Claims {
                    sub: username,
                    exp: 10000000000, // Set expiration as needed
                };
                let token = jsonwebtoken::encode(&jsonwebtoken::Header::default(), &claims, &jsonwebtoken::EncodingKey::from_secret(SECRET.as_ref()))
                    .map_err(|_| HttpResponse::InternalServerError().body("Error generating token"));
                if let Ok(token) = token {
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
            error!("Error: {:?}", e);
            HttpResponse::InternalServerError().body("Error logging in")
        }
    }
}

#[post("/register-vehicle")]
pub async fn register_vehicle(
    vehicle: web::Json<VehicleRegistrationInfo>,
    data: web::Data<dyn UserService>, 
    req: HttpRequest,
) -> impl Responder {
    let token_value = extract_token(&req).unwrap_or_default();

    let decoded = decode_token(&token_value);
    let email = decoded.unwrap().sub;

    info!("Received vehicle registration request for user: {}", email);
    match data.get_user(&email).await {
        Ok(Some(mut db_user)) => {
            // Handle vehicle registration logic
            if let Some(ref mut vehicle_numbers) = db_user.vehicle_number {
                vehicle_numbers.extend(vehicle.vehicle_number.clone());
            } else {
                db_user.vehicle_number = Some(vehicle.vehicle_number.clone());
            }
            db_user.vehicle_type = Some(vehicle.vehicle_type.clone());
            match data.update_user(db_user).await {
                Ok(_) => HttpResponse::Created().finish(),
                Err(e) => {
                    error!("Error updating user: {:?}", e);
                    HttpResponse::InternalServerError().body("Error registering vehicle")
                }
            }
        }
        Ok(None) => HttpResponse::Unauthorized().body("User not found"),
        Err(e) => {
            error!("Error retrieving user: {:?}", e);
            HttpResponse::InternalServerError().body("Error registering vehicle")
        }
    }
}

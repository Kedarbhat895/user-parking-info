use actix_web::{get, web, HttpRequest, HttpResponse, Responder};
use crate::models::{VehicleType};
use crate::services::LotService;
use crate::utils::auth::{extract_token, decode_token}; 
use log::{info, error};
use std::str::FromStr;


#[get("/available-slots/vehicle_type/{vehicle_type}")]
pub async fn get_available_slots(
    req: HttpRequest,
    path: web::Path<String>,
    lot_service: web::Data<dyn LotService>,
) -> impl Responder {
    let vehicle_type_str = path.into_inner();
    info!("Received request to get available slots with param: {}", vehicle_type_str);

    let vehicle_type = match VehicleType::from_str(&vehicle_type_str) {
        Ok(v) => v,
        Err(_) => return HttpResponse::BadRequest().body("Invalid vehicle type"),
    };

    
    let token_value = extract_token(&req).unwrap_or_default();

    let decoded = decode_token(&token_value);
    let email = decoded.unwrap().sub;

    

    // Find lots using the service
    info!("Received request to get available slots for vehicle type: {} from user: {}", vehicle_type, email);
    let lots = lot_service.find_lots(vehicle_type).await;
    match lots {
        Ok(lots) => HttpResponse::Ok().json(lots),
        Err(e) => {
            error!("Error: {:?}", e);
            HttpResponse::InternalServerError().body("Error finding available slots")
        }
    }
}

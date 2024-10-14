use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use super::vehicle_type::VehicleType;

#[derive(Debug, Serialize, Deserialize)]
pub struct ParkingLot {
    pub parking_lot_id: String,
    pub slot_id: String,
    pub status: bool,
    pub email: Option<String>,
    pub slot_type: VehicleType,             
    pub reservation_time: Option<DateTime<Utc>>,
}
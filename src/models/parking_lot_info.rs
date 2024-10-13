use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::vehicle_type::VehicleType;

#[derive(Debug, Serialize, Deserialize)]
pub struct ParkingLot {
    pub parking_lot_id: u64,
    pub slot_id: u64,
    pub status: bool,
    pub email: String,
    pub slot_type: VehicleType,             
    pub reservation_time: Option<DateTime<Utc>>,
}

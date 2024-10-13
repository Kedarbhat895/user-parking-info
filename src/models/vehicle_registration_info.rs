use serde::{Deserialize, Serialize};
use std::vec::Vec;
use super::vehicle_type::VehicleType;

#[derive(Debug, Serialize, Deserialize)]
pub struct VehicleRegistrationInfo {
    pub vehicle_number: Vec<String>,  // A list of vehicle numbers
    pub vehicle_type: VehicleType,    // Using VehicleType enum
}

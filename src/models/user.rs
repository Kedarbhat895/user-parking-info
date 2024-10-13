use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::vec::Vec;
use super::vehicle_type::VehicleType;

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub username: String,
    pub password: String,
    pub email: String,
    pub vehicle_number: Option<Vec<String>>, // Now optional
    pub vehicle_type: Option<VehicleType>,   // Now optional
    pub reservation: Option<HashMap<String, String>>, // Now optional
}
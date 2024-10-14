use std::str::FromStr;
use std::fmt;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum VehicleType {
    SMALL,
    MEDIUM,
    LARGE,
}

impl FromStr for VehicleType {
    type Err = String;
    
    fn from_str(s: &str) -> Result<VehicleType, Self::Err> {
        match s.to_uppercase().as_str() {
            "SMALL" => Ok(VehicleType::SMALL),
            "MEDIUM" => Ok(VehicleType::MEDIUM),
            "LARGE" => Ok(VehicleType::LARGE),
            _ => Err(format!("'{}' is not a valid VehicleType", s)),
        }
    }
}

impl fmt::Display for VehicleType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                VehicleType::SMALL => "Small",
                VehicleType::MEDIUM => "Medium",
                VehicleType::LARGE => "Large",
            }
        )
    }
}
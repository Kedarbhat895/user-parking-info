use async_trait::async_trait;
use aws_sdk_dynamodb::Error;
use crate::models::VehicleType;
use std::vec::Vec;


#[async_trait]
pub trait LotService {
    async fn find_lots(&self, vehicle_type: VehicleType) -> Result<Vec<String>, Error>;
    // async fn reserve(&self, lot_id: &str, vehicle_number: &str, vehicle_type: VehicleType) -> Result<Option<String>, Error>;
}
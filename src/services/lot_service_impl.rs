use async_trait::async_trait;
use crate::services::LotService;
use aws_sdk_dynamodb::{Client, Error};
use aws_sdk_dynamodb::types::AttributeValue;
use crate::models::VehicleType;
use std::vec::Vec;
use std::sync::Arc;


#[derive(Clone)]
pub struct LotServiceImpl {
    pub client: Arc<Client>,
    pub table_name: String,
}

#[async_trait]
impl LotService for LotServiceImpl {
    async fn find_lots(&self, vehicle_type: VehicleType) -> Result<Vec<String>, Error> {
        // Build the query to DynamoDB

        println!("Finding tables...");
        let result = self.client
            .query()
            .table_name(&self.table_name)
            .key_condition_expression("parking_lot_id = :lotId")
            .filter_expression("#s = :status AND slot_type = :vehicleType")
            .expression_attribute_names("#s", "status")
            .expression_attribute_values(
                ":lotId", AttributeValue::S("1".to_string()),
            )
            .expression_attribute_values(
                ":status", AttributeValue::Bool(false),
            )
            .expression_attribute_values(
                ":vehicleType", AttributeValue::S(vehicle_type.to_string()),
            )
            .send()
            .await?;

        // Process the response to extract slot IDs
        let mut slots = Vec::new();
        if let Some(items) = result.items {
            for item in items {
                if let Some(slot_id) = item.get("slot_id").and_then(|val| val.as_s().ok()) {
                    slots.push(slot_id.clone());
                }
            }
        }

        Ok(slots)
    }
}


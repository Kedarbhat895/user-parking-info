use async_trait::async_trait;
use crate::models::User;
use crate::services::user_service::UserService;
use aws_sdk_dynamodb::{Client, Error};
use aws_sdk_dynamodb::types::AttributeValue;
use crate::models::VehicleType;
use std::str::FromStr;
use std::collections::HashMap;

#[derive(Clone)]
pub struct UserServiceImpl {
    pub client: Client,
    pub table_name: String,
}


#[async_trait]
impl UserService for UserServiceImpl {
    async fn update_user(&self, user: User) -> Result<(), Error> {
        let mut item = HashMap::new();
        item.insert("username".to_string(), AttributeValue::S(user.username));
        item.insert("password".to_string(), AttributeValue::S(user.password));
        item.insert("email".to_string(), AttributeValue::S(user.email));
        if let Some(vehicle_numbers) = user.vehicle_number {
            item.insert("vehicle_number".to_string(), AttributeValue::Ss(vehicle_numbers));
        }
        if let Some(vehicle_type) = user.vehicle_type {
            item.insert("vehicle_type".to_string(), AttributeValue::S(vehicle_type.to_string()));
        }
        
        self.client.put_item()
            .table_name(&self.table_name)
            .set_item(Some(item))
            .send()
            .await?;
        Ok(())
    }


    async fn get_user(&self, email: &str) -> Result<Option<User>, Error> {
        let result = self.client.get_item()
            .table_name(&self.table_name)
            .key("email", AttributeValue::S(email.to_string()))
            .send()
            .await?;

        if let Some(item) = result.item {
            let user = User {
                username: item.get("username")
                    .and_then(|v| v.as_s().ok().map(String::from))
                    .unwrap_or_default(),
                password: item.get("password")
                    .and_then(|v| v.as_s().ok().map(String::from))
                    .unwrap_or_default(),
                email: item.get("email")
                    .and_then(|v| v.as_s().ok().map(String::from))
                    .unwrap_or_default(),
                vehicle_number: item.get("vehicle_number")
                    .and_then(|v| v.as_ss().ok())
                    .map(|vec| vec.iter().map(String::from).collect()),
                vehicle_type: item.get("vehicle_type")
                    .and_then(|v| v.as_s().ok())
                    .and_then(|s| VehicleType::from_str(s).ok()),
                reservation: item.get("reservation")
                    .and_then(|v| v.as_m().ok())
                    .map(|m| m.iter().map(|(k, v)| (k.to_string(), v.as_s().map(String::from).unwrap_or_default())).collect()),
            };
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }
}

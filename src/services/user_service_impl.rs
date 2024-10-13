use async_trait::async_trait;
use crate::models::User;
use crate::services::user_service::UserService;
use aws_sdk_dynamodb::{Client, Error};
use aws_sdk_dynamodb::types::AttributeValue;

#[derive(Clone)]
pub struct UserServiceImpl {
    pub client: Client,
    pub table_name: String,
}


#[async_trait]
impl UserService for UserServiceImpl {
    async fn create_user(&self, user: User) -> Result<(), Error> {
        self.client.put_item()
            .table_name(&self.table_name)
            .item("username", AttributeValue::S(user.username))
            .item("password", AttributeValue::S(user.password))
            .item("email", AttributeValue::S(user.email))
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
                    .and_then(|v| Some(v.as_s())) // Only attempt conversion if value exists
                    .map(|v| v.expect("REASON").to_string()) // Convert to string if conversion is successful
                    .unwrap_or_else(|| "".to_string()), // Default to empty string if missing
                password: item.get("password")
                .and_then(|v| Some(v.as_s())) // Only attempt conversion if value exists
                .map(|v| v.expect("REASON").to_string())
                    .unwrap_or_else(|| "".to_string()),
                email: item.get("email")
                .and_then(|v| Some(v.as_s())) // Only attempt conversion if value exists
                .map(|v| v.expect("REASON").to_string())
                    .unwrap_or_else(|| "".to_string()),
            };
            Ok(Some(user))
        } else {
            Ok(None)
        }
    }

    async fn insert_sample_user(&self) -> Result<(), Error> {
        let sample_user = User {
            username: "sample_user".to_string(),
            password: "sample_password".to_string(),
            email: "sample_user@example.com".to_string(),
        };
        println!("Inserting sample user...");
        self.create_user(sample_user).await
    }
    
}



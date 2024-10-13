use async_trait::async_trait;
use crate::models::User;
use aws_sdk_dynamodb::Error;

#[async_trait]
pub trait UserService {
    async fn insert_sample_user(&self) -> Result<(), Error>;
    async fn create_user(&self, user: User) -> Result<(), Error>;
    async fn get_user(&self, email: &str) -> Result<Option<User>, Error>;
}
use crate::view_models::User;
use async_trait::async_trait;
use std::sync::Arc;

pub type DynUserRepo = Arc<dyn UserRepo + Send + Sync>;

pub type UserRepoError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[async_trait]
pub trait UserRepo {
    async fn find(&self, card_num: String) -> Result<User, UserRepoError>;

    async fn create(&self) -> Result<User, UserRepoError>;
}

pub struct UserRepoImpl;

#[async_trait]
impl UserRepo for UserRepoImpl {
    async fn find(&self, _card_num: String) -> Result<User, UserRepoError> {
        Ok(User {
            username: "root".to_string(),
            password: "root".to_string(),
        })
    }

    async fn create(&self) -> Result<User, UserRepoError> {
        Ok(User {
            username: "root".to_string(),
            password: "root".to_string(),
        })
    }
}

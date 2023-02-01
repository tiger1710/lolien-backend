use std::sync::Arc;

use super::{user_domain::Model, UserRepository};
use async_trait::async_trait;
use sea_orm::DbErr;

pub struct UserUsecaseImpl {
    user_repository: Arc<dyn UserRepository>,
}

impl UserUsecaseImpl {
    pub const fn new(user_repository: Arc<dyn UserRepository>) -> UserUsecaseImpl {
        UserUsecaseImpl { user_repository }
    }
}

#[async_trait]
impl super::UserUsecase for UserUsecaseImpl {
    async fn get_by_id(&self, uid: i32) -> Result<Option<Model>, DbErr> {
        self.user_repository.select_user_by_id(uid).await
    }
}

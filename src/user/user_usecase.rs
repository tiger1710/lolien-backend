use std::sync::Arc;

use actix_web::web;
use async_trait::async_trait;

use super::UserRepository;

use entity::user::{ActiveModel, Model};

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
    async fn get_by_id(&self, uid: i32) -> anyhow::Result<Option<Model>> {
        Ok(self.user_repository.select_user_by_id(uid).await?)
    }

    async fn create_user(&self, form: web::Json<Model>) -> anyhow::Result<ActiveModel> {
        let form = form.into_inner();
        Ok(self.user_repository.insert_user(form).await?)
    }
}

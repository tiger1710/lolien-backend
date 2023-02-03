use std::sync::Arc;

use super::{user_domain::Model as User, ActiveModel, UserRepository};
use actix_web::web;
use async_trait::async_trait;

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
    async fn get_by_id(&self, uid: i32) -> Option<User> {
        if let Ok(res) = self.user_repository.select_user_by_id(uid).await {
            res
        } else {
            None
        }
    }

    async fn create_user(&self, form: web::Json<User>) -> Option<ActiveModel> {
        let form = form.into_inner();
        if let Ok(res) = self.user_repository.insert_user(form).await {
            Some(res)
        } else {
            None
        }
    }
}

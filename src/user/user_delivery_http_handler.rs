use std::sync::Arc;

use async_trait::async_trait;

use actix_web::{error::ErrorNotFound, Error, HttpResponse};

use super::UserUsecase;

pub struct UserHttpHandlerImpl {
    user_usecase: Arc<dyn UserUsecase>,
}

impl UserHttpHandlerImpl {
    pub const fn new(user_usecase: Arc<dyn UserUsecase>) -> UserHttpHandlerImpl {
        UserHttpHandlerImpl { user_usecase }
    }
}

#[async_trait]
impl super::UserHttpHandler for UserHttpHandlerImpl {
    async fn get_user(&self, uid: i32) -> Result<HttpResponse, Error> {
        match self.user_usecase.get_by_id(uid).await {
            Ok(user) => Ok(HttpResponse::Ok().json(user)),
            Err(_) => Err(ErrorNotFound("User not found.")),
        }
    }

    async fn hello(&self) -> Result<HttpResponse, Error> {
        Ok(HttpResponse::Ok().body("Hello world"))
    }
}

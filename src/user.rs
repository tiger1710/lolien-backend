use async_trait::async_trait;

mod user_delivery_http_handler;
pub mod user_domain;
mod user_repository;
mod user_usecase;

use actix_web::{get, web, Error, HttpResponse};
use sea_orm::DbErr;

use self::{
    user_delivery_http_handler::UserHttpHandlerImpl, user_domain::Model,
    user_repository::UserRepositoryImpl, user_usecase::UserUsecaseImpl,
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn select_user_by_id(&self, uid: i32) -> Result<Option<Model>, DbErr>;
}

#[async_trait]
pub trait UserUsecase: Send + Sync {
    async fn get_by_id(&self, uid: i32) -> Result<Option<Model>, DbErr>;
}

#[async_trait]
pub trait UserHttpHandler: Send + Sync {
    async fn get_user(&self, uid: i32) -> Result<HttpResponse, Error>;
    async fn hello(&self) -> Result<HttpResponse, Error>;
}

use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserContainer {
    http_delivery: Arc<dyn UserHttpHandler>,
}

impl UserContainer {
    pub fn new_user_container(conn: DatabaseConnection) -> UserContainer {
        let http_delivery = Arc::new(UserHttpHandlerImpl::new(Arc::new(UserUsecaseImpl::new(
            Arc::new(UserRepositoryImpl::new(conn)),
        ))));

        UserContainer { http_delivery }
    }
}

#[get("/users/{user_id}")]
async fn get_user(
    data: web::Data<UserContainer>,
    uid: web::Path<i32>,
) -> Result<HttpResponse, Error> {
    let delivery = &data.http_delivery;
    delivery.get_user(*uid).await
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(get_user);
}

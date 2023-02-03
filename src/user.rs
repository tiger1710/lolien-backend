use async_trait::async_trait;

mod user_delivery_http_handler;
mod user_domain;
mod user_repository;
mod user_usecase;

use actix_web::web;
use sea_orm::DbErr;

use self::{
    user_delivery_http_handler::UserHttpHandler,
    user_domain::{ActiveModel, Model},
    user_repository::UserRepositoryImpl,
    user_usecase::UserUsecaseImpl,
};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn select_user_by_id(&self, uid: i32) -> Result<Option<Model>, DbErr>;
    async fn insert_user(&self, user: Model) -> Result<ActiveModel, DbErr>;
}

#[async_trait]
pub trait UserUsecase: Send + Sync {
    async fn get_by_id(&self, uid: i32) -> Option<Model>;
    async fn create_user(&self, form: web::Json<Model>) -> Option<ActiveModel>;
}

use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct UserContainer {
    http_delivery: UserHttpHandler,
}

impl UserContainer {
    pub fn new_user_container(conn: DatabaseConnection) -> UserContainer {
        let http_delivery = UserHttpHandler::new(Arc::new(UserUsecaseImpl::new(Arc::new(
            UserRepositoryImpl::new(conn),
        ))));

        UserContainer { http_delivery }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(user_delivery_http_handler::get_user);
    cfg.service(user_delivery_http_handler::create_user);
}

use async_trait::async_trait;

mod replay_delivery_http_handler;
mod replay_domain;
mod replay_repository;
mod replay_usecase;

use actix_web::web::{self, Bytes};
use rofl_parser::rofl::rofl_json::RoflJson;

use self::{
    replay_delivery_http_handler::ReplayHttpHandler, replay_repository::ReplayRepositoryImpl,
    replay_usecase::ReplayUsecaseImpl,
};

#[async_trait]
pub trait ReplayRepository: Send + Sync {}

#[async_trait]
pub trait ReplayUsecase: Send + Sync {
    async fn get_json_from_rofl(&self, data: &[u8]) -> anyhow::Result<RoflJson>;
}

use sea_orm::DatabaseConnection;
use std::sync::Arc;

#[derive(Clone)]
pub struct ReplayContainer {
    http_delivery: ReplayHttpHandler,
}

impl ReplayContainer {
    pub fn new_replay_container(conn: DatabaseConnection) -> ReplayContainer {
        let replay_repository = ReplayRepositoryImpl::new(conn);
        let replay_usecase = ReplayUsecaseImpl::new(Arc::new(replay_repository));
        let replay_http_handler = ReplayHttpHandler::new(Arc::new(replay_usecase));
        ReplayContainer { http_delivery: replay_http_handler }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(replay_delivery_http_handler::upload_replay);
}

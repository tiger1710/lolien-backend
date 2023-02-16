use async_trait::async_trait;

use entity::match_info;
use entity::participant;

mod replay_delivery_http_handler;
mod replay_repository;
mod replay_usecase;

use actix_web::web;
use rofl_parser::rofl::rofl_json::RoflJson;

use self::{
    replay_delivery_http_handler::ReplayHttpHandler, replay_repository::ReplayRepositoryImpl,
    replay_usecase::ReplayUsecaseImpl,
};

#[async_trait]
pub trait ReplayRepository: Send + Sync {
    async fn insert_match_info(
        &self,
        match_info: match_info::ActiveModel,
    ) -> anyhow::Result<InsertResult<match_info::ActiveModel>>;
    async fn insert_many_participants(
        &self,
        participants: Vec<participant::ActiveModel>,
    ) -> anyhow::Result<InsertResult<participant::ActiveModel>>;
}

#[async_trait]
pub trait ReplayUsecase: Send + Sync {
    async fn get_json_from_rofl(&self, data: &[u8]) -> anyhow::Result<RoflJson>;
    async fn create_match_info(&self, file_name: &str) -> anyhow::Result<()>;
}

use sea_orm::{DatabaseConnection, InsertResult};
use std::sync::Arc;

#[derive(Clone)]
pub struct ReplayContainer {
    http_delivery: ReplayHttpHandler,
}

impl ReplayContainer {
    pub fn new(conn: DatabaseConnection) -> ReplayContainer {
        let replay_repository = ReplayRepositoryImpl::new(conn);
        let replay_usecase = ReplayUsecaseImpl::new(Arc::new(replay_repository));
        let replay_http_handler = ReplayHttpHandler::new(Arc::new(replay_usecase));
        ReplayContainer {
            http_delivery: replay_http_handler,
        }
    }
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(replay_delivery_http_handler::upload_replay);
}

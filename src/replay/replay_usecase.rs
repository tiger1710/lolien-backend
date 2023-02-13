use std::sync::Arc;

use super::ReplayRepository;
use anyhow::Ok;
use async_trait::async_trait;
use rofl_parser::rofl::{self, rofl_json::RoflJson};

pub struct ReplayUsecaseImpl {
    replay_repository: Arc<dyn ReplayRepository>,
}

impl ReplayUsecaseImpl {
    pub const fn new(replay_repository: Arc<dyn ReplayRepository>) -> ReplayUsecaseImpl {
        ReplayUsecaseImpl { replay_repository }
    }
}

#[async_trait]
impl super::ReplayUsecase for ReplayUsecaseImpl {
    async fn get_json_from_rofl(&self, data: &[u8]) -> anyhow::Result<RoflJson> {
        let mut rofl = rofl::Rofl::new();
        rofl.parse_rofl_data(data).expect("Can't parse rofl data.");

        if let Some(rofl_json) = rofl.get_rofl_json() {
            Ok(rofl_json.clone())
        } else {
            Err(anyhow::anyhow!("rofl json is none."))
        }
    }
}

use std::sync::Arc;

use super::{ReplayRepository, Match};
use async_trait::async_trait;
use entity::{match_info, participant};
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

    async fn create_match_info(&self, file_name: &str) -> anyhow::Result<()> {
        // parse file name
        let pos = file_name.find(".rofl").unwrap_or(0);
        let match_id = &file_name[..pos];
        let match_id = match_id.replace('-', "_");

        let res = riot_api::get_match_info(&match_id).await?;

        let match_info = match_info::ActiveModel::from(&res);
        let match_id = res.metadata.matchId.as_str();

        let participants = res
            .info
            .participants
            .iter()
            .map(|p| participant::ActiveModel::from((match_id, p)))
            .collect();

        // insert match_info & participants
        self.replay_repository.insert_match_info(match_info).await?;
        self.replay_repository
            .insert_many_participants(participants)
            .await?;

        Ok(())
    }
}

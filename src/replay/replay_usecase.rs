use std::sync::Arc;

use super::ReplayRepository;
use async_trait::async_trait;
use entity::{match_info, participant};
use rofl_parser::rofl::{self, rofl_json::RoflJson};
use sea_orm::Set;

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
        let pos = file_name.find(".rofl").unwrap_or(0);
        let match_id = &file_name[..pos];
        let match_id = match_id.replace("-", "_");

        let res = riot_api::get_match_info(&match_id).await?;

        let match_info = match_info::ActiveModel::from(&res);
        let mut participants = Vec::new();
        for p in res.info.participants {
            participants.push(participant::ActiveModel {
                match_id: Set(res.metadata.matchId.clone()),
                kills: Set(Some(p.kills)),
                deaths: Set(Some(p.deaths)),
                assists: Set(Some(p.assists)),
                champ_level: Set(Some(p.champLevel)),
                champion_id: Set(Some(p.championId)),
                champion_name: Set(Some(p.championName)),
                champion_transform: Set(Some(p.championTransform)),
                summoner_spell1_id: Set(Some(p.summoner1Id)),
                summoner_spell2_id: Set(Some(p.summoner2Id)),
                summoner_name: Set(Some(p.summonerName)),
                item0: Set(Some(p.item0)),
                item1: Set(Some(p.item1)),
                item2: Set(Some(p.item2)),
                item3: Set(Some(p.item3)),
                item4: Set(Some(p.item4)),
                item5: Set(Some(p.item5)),
                item6: Set(Some(p.item6)),
                primary_perk_selection1: Set(Some(p.perks.styles[0].selections[0].perk)),
                primary_perk_selection2: Set(Some(p.perks.styles[0].selections[1].perk)),
                primary_perk_selection3: Set(Some(p.perks.styles[0].selections[2].perk)),
                primary_perk_selection4: Set(Some(p.perks.styles[0].selections[3].perk)),
                sub_perk_selection1: Set(Some(p.perks.styles[1].selections[0].perk)),
                sub_perk_selection2: Set(Some(p.perks.styles[1].selections[1].perk)),
                stat_perk_offense: Set(Some(p.perks.statPerks.offense)),
                stat_perk_defense: Set(Some(p.perks.statPerks.defense)),
                stat_perk_flex: Set(Some(p.perks.statPerks.flex)),
                total_damage_dealt_to_champions: Set(Some(p.totalDamageDealtToChampions)),
                total_damage_taken: Set(Some(p.totalDamageTaken)),
                total_minions_killed: Set(Some(p.totalMinionsKilled)),
                gold_earned: Set(Some(p.goldEarned)),
                team_position: Set(Some(p.teamPosition)),
                vision_score: Set(Some(p.visionScore)),
                vision_wards_bought_in_game: Set(Some(p.visionWardsBoughtInGame)),
                wards_killed: Set(Some(p.wardsKilled)),
                wards_placed: Set(Some(p.wardsPlaced)),
                team_id: Set(Some(p.teamId)),
            })
        }

        self.replay_repository.insert_match_info(match_info).await?;
        self.replay_repository
            .insert_many_participants(participants)
            .await?;

        Ok(())
    }
}

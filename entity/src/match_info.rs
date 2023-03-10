//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.0

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "match_info")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false, unique)]
    pub match_id: String,
    pub game_creation: Option<i64>,
    pub game_duration: Option<i32>,
    pub game_start_timestamp: Option<i64>,
    pub game_end_timestamp: Option<i64>,
    pub game_id: Option<i64>,
    pub game_mod: Option<String>,
    pub game_type: Option<String>,
    pub game_version: Option<String>,
    pub map_id: Option<i32>,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::participant::Entity")]
    Participant,
}

impl Related<super::participant::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Participant.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}

impl From<&riot_api::Match> for ActiveModel {
    fn from(value: &riot_api::Match) -> Self {
        use sea_orm::Set;
        ActiveModel {
            match_id: Set(value.metadata.matchId.clone()),
            game_creation: Set(Some(value.info.gameCreation)),
            game_duration: Set(Some(value.info.gameDuration)),
            game_start_timestamp: Set(Some(value.info.gameStartTimestamp)),
            game_end_timestamp: Set(Some(value.info.gameEndTimestamp)),
            game_id: Set(Some(value.info.gameId)),
            game_mod: Set(Some(value.info.gameMode.clone())),
            game_type: Set(Some(value.info.gameType.clone())),
            game_version: Set(Some(value.info.gameVersion.clone())),
            map_id: Set(Some(value.info.mapId)),
        }
    }
}

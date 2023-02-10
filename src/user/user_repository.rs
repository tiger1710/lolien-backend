use super::user_domain::{ActiveModel, Entity, Model};
use async_trait::async_trait;

use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, DatabaseConnection, DbErr, EntityTrait, Set};

pub struct UserRepositoryImpl {
    conn: DatabaseConnection,
}

impl UserRepositoryImpl {
    pub const fn new(conn: DatabaseConnection) -> UserRepositoryImpl {
        UserRepositoryImpl { conn }
    }
}

#[async_trait]
impl super::UserRepository for UserRepositoryImpl {
    async fn select_user_by_id(&self, uid: i32) -> anyhow::Result<Option<Model>> {
        Ok(Entity::find_by_id(uid).one(&self.conn).await?)
    }

    async fn insert_user(&self, user: Model) -> Result<ActiveModel, DbErr> {
        Ok(ActiveModel {
            id: NotSet,
            name: Set(user.name.to_owned()),
        }
        .save(&self.conn)
        .await?)
    }
}

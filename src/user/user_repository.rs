use async_trait::async_trait;
use entity::user::{ActiveModel, Entity as User, Model};

use sea_orm::{ActiveModelTrait, ActiveValue::NotSet, DatabaseConnection, EntityTrait, Set};

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
        Ok(User::find_by_id(uid).one(&self.conn).await?)
    }

    async fn insert_user(&self, user: Model) -> anyhow::Result<ActiveModel> {
        Ok(ActiveModel {
            id: NotSet,
            name: Set(user.name.to_owned()),
        }
        .save(&self.conn)
        .await?)
    }
}

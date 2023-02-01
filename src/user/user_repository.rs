use super::user_domain::Entity as User;
use crate::user::Model;
use async_trait::async_trait;

use sea_orm::{DatabaseConnection, DbErr, EntityTrait};

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
    async fn select_user_by_id(&self, uid: i32) -> Result<Option<Model>, DbErr> {
        Ok(User::find_by_id(uid).one(&self.conn).await?)
    }
}

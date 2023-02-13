use async_trait::async_trait;

use sea_orm::DatabaseConnection;

pub struct ReplayRepositoryImpl {
    conn: DatabaseConnection,
}

impl ReplayRepositoryImpl {
    pub const fn new(conn: DatabaseConnection) -> ReplayRepositoryImpl {
        ReplayRepositoryImpl { conn }
    }
}

#[async_trait]
impl super::ReplayRepository for ReplayRepositoryImpl {}

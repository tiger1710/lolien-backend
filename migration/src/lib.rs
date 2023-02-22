pub use sea_orm_migration::prelude::*;

mod m20230214_000001_create_table;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![Box::new(m20230214_000001_create_table::Migration)]
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn create_user_table() {
        use crate::m20230214_000001_create_table::User;

        let mut statement = Table::create();
        statement
            .table(User::Table)
            .if_not_exists()
            .col(
                ColumnDef::new(User::Id)
                    .integer()
                    .not_null()
                    .auto_increment(),
            )
            .col(ColumnDef::new(User::Name).string_len(32).not_null())
            .primary_key(Index::create().name("pk_user").col(User::Id));

        assert_eq!(
            statement.to_string(MysqlQueryBuilder),
            [
                "CREATE TABLE IF NOT EXISTS `user` (",
                "`id` int NOT NULL AUTO_INCREMENT,",
                "`name` varchar(32) NOT NULL,",
                "PRIMARY KEY `pk_user` (`id`)",
                ")",
            ]
            .join(" ")
        );
    }
}

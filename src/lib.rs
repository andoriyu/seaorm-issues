pub mod user {
    use sea_orm::entity::prelude::*;

    #[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
    #[sea_orm(table_name = "user")]
    pub struct Model {
        #[sea_orm(primary_key)]
        pub user_id: i32,
    }

    #[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
    pub enum Relation {}

    impl ActiveModelBehavior for ActiveModel {}
}

#[cfg(test)]
mod tests {
    use sea_orm::Statement;
    use sea_orm::Set;
    use sea_orm::entity::prelude::*;
    use sea_orm::Database;
    use sea_orm::ConnectOptions;
    use sea_orm::sea_query::OnConflict;

    async fn setup() -> DatabaseConnection {
        let opt = ConnectOptions::new("sqlite::memory:".to_string());
        let db = Database::connect(opt).await.unwrap();

        db.execute(Statement::from_string(
                db.get_database_backend(),
                "CREATE TABLE user (user_id int primary key);".to_owned(),
                ))
            .await
            .unwrap();
        db
    }
    #[tokio::test]
    async fn on_conflict() {
        let db = setup().await;
        for _ in 0..2 {
            let record = super::user::ActiveModel {
                user_id: Set(1)
            };
            let on_conflict = OnConflict::columns([super::user::Column::UserId])
                .do_nothing()
                .to_owned();
            super::user::Entity::insert_many(vec![record])
                .on_conflict(on_conflict)
                .exec(&db)
                .await.unwrap();
        }
    }
}

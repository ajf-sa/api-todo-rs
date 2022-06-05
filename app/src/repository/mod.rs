pub mod repository {

    use sqlx::prelude::*;
    use sqlx::PgPool;

    use super::user::User;

    #[derive(Debug, Clone)]
    pub struct Repository {
        pool: PgPool,
        // table_name: &'static str,
    }

    trait RepositoryInterface {
        fn new(pool: PgPool) -> Self;
        fn set_user(&self, name: String) -> Result<bool, sqlx::Error>;
        fn get_users(&self) -> Result<Vec<User>, sqlx::Error>;
    }

    impl Repository {
        pub fn new(pool: PgPool) -> Repository {
            Repository {
                pool,
                // table_name: "users",
            }
        }
        pub async fn set_user(&self, name: String) -> Result<bool, sqlx::Error> {
            let stmt: &str = "INSERT INTO users (name) VALUES ($1)";
            self.pool.execute(sqlx::query(stmt).bind(name)).await?;
            Ok(true)
        }
        pub async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
            let stmt = "SELECT name FROM users";
            let mut users = Vec::<User>::new();
            let rows = sqlx::query(stmt).fetch_all(&self.pool).await?;
            for row in rows {
                let name = row.get("name");
                users.push(User { name });
            }
            Ok(users)
        }
    }
}

pub mod user {
    use serde::{Deserialize, Serialize};
    use sqlx::FromRow;

    #[derive(Debug, FromRow, Serialize, Deserialize)]
    pub struct User {
        pub name: String,
    }
}

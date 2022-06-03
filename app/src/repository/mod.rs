pub mod repository {

    use sqlx::prelude::*;
    use sqlx::PgPool;

    use super::user::User;

    pub(crate) struct Repository {
        pool: PgPool,
    }

    impl Repository {
        pub fn new(pool: PgPool) -> Repository {
            Repository { pool }
        }
        pub async fn set_user(&self, name: String) -> Result<bool, sqlx::Error> {
            self.pool
                .execute(sqlx::query("INSERT INTO users (name) VALUES ($1)").bind(name))
                .await?;
            Ok(true)
        }
        pub async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
            let mut users = Vec::<User>::new();
            let rows = sqlx::query("select name from users")
                .fetch_all(&self.pool)
                .await?;
            for row in rows {
                let name = row.get("name");
                users.push(User { name });
            }
            Ok(users)
        }
    }
}

pub mod user {
    use sqlx::{FromRow, Row};

    #[derive(Debug, FromRow)]
    pub struct User {
        pub name: String,
    }
}

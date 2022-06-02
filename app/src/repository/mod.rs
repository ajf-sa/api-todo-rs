pub mod repository {
    use futures::future::ok;
    use sqlx::{Pool, Postgres};

    use super::user::User;

    pub(crate) struct Repository {
        pool: Pool<Postgres>,
    }

    impl Repository {
        pub fn new(pool: Pool<Postgres>) -> Repository {
            Repository { pool }
        }
        pub async fn get_users(&self) -> Result<Vec<User>, sqlx::Error> {
            Ok(vec![User {
                name: "test".to_string(),
            }])
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

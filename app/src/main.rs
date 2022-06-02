use futures::TryStreamExt;
use sqlx::{Executor, FromRow, Row};
mod repository;
mod utilts;

#[derive(Debug, FromRow)]
struct User {
    pub name: String,
}
#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let env = utilts::env::Env::new();
    let conn = utilts::connect::Connect::new();
    let pool = match conn.await.connection().await {
        Ok(pool) => pool,
        Err(e) => panic!("{}", e),
    };

    let res = repository::repository::Repository::new(pool);
    let users = res.get_users().await?;
    Ok(())
}

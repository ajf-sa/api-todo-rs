mod repository;
mod utilts;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let conn = utilts::connect::Connect::new();
    let res = repository::repository::Repository::new(conn.connect().await?);
    res.set_user("jaiz".to_string()).await?;
    let users = res.get_users().await?;
    println!("{:?}", users);
    Ok(())
}

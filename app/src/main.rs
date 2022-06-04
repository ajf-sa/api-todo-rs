use std::fmt::Error;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use futures::future::ok;
use repository::user::User;

mod repository;
mod todo;
mod utilts;

#[tokio::main]
async fn main() -> Result<(), sqlx::Error> {
    let conn = utilts::connect::Connect::new();
    let res = repository::repository::Repository::new(conn.connect().await?);
    let todo = todo::service::Service::new(res);
    let users: Vec<User> = todo.get_users().await?;
    let mut hnd = todo::handler::Handler::new(todo);
    Ok(())
}

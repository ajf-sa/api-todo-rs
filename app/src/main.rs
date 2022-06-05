use actix_web::{web, App, HttpServer};
use anyhow::Result;

use todo::{handler::ActixSchema, service};
mod repository;
mod todo;
mod utilts;

#[tokio::main]

async fn main() -> Result<()> {
    let env = utilts::env::Env::new();
    let conn = utilts::connect::Connect::new();
    let res = repository::repository::Repository::new(conn.connect().await?);
    let mut todo = service::Service::new(res);
    HttpServer::new(move || {
        App::new()
            .data(ActixSchema {
                service: todo.clone(),
            })
            .route("/get", web::get().to(todo::handler::get_names))
            .route("/{list_id}", web::get().to(todo::handler::add_name))
    })
    .bind(("0.0.0.0", env.get_app_port()))?
    .run()
    .await?;

    Ok(())
}

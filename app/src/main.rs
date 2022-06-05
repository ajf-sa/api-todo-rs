use actix_web::{web, App, HttpServer, Responder};
use anyhow::Result;
use futures::TryFutureExt;
use repository::user::User;
use todo::handler::ActixSchema;
mod repository;
mod todo;
mod utilts;

#[tokio::main]

async fn main() -> Result<()> {
    let conn = utilts::connect::Connect::new();
    let res = repository::repository::Repository::new(conn.connect().await?);
    let mut todo = todo::service::Service::new(res);

    // let mut hnd = todo::handler::Handler::new(todo.clone());
    HttpServer::new(move || {
        App::new()
            .data(ActixSchema {
                service: todo.clone(),
            })
            .route("/get", web::get().to(todo::handler::get_names))
            .route("/{list_id}", web::get().to(todo::handler::add_name))
    })
    .bind(("0.0.0.0", 8081))?
    .run()
    .await?;

    Ok(())
}

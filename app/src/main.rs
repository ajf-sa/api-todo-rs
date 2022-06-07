use anyhow::Result;

use api::{
    repository::repository::Repository,
    startup::Application,
    todo::{handler::ActixSchema, service},
    utilts,
};

#[tokio::main]

async fn main() -> Result<()> {
    let env = utilts::env::Env::new();
    let conn = utilts::connect::Connect::new();
    let res = Repository::new(conn.connect().await?);
    let todo = service::Service::new(res.clone());
    let schema = ActixSchema {
        service: todo,
        repository: res,
        env: env.clone(),
    };
    let app = Application::build(schema, env).unwrap();
    app.run_until_stopped().await?;

    Ok(())
}

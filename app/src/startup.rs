use std::net::TcpListener;

use crate::{
    todo::handler::{add_name, get_names, ActixSchema},
    utilts::env::Env,
};
use actix_server::Server;

use actix_web::{web, App, HttpServer};

pub struct Application {
    pub app_schema: ActixSchema,
    pub server: Server,
}

impl Application {
    pub fn build(schema: ActixSchema, env: Env) -> Result<Self, std::io::Error> {
        let address = format!("{}:{}", "0.0.0.0", env.get_app_port());
        let listener = TcpListener::bind(&address)?;
        let server = run(schema.clone(), listener)?;
        Ok(Self {
            app_schema: schema,
            server,
        })
    }

    pub async fn run_until_stopped(self) -> Result<(), std::io::Error> {
        self.server.await
    }
}

pub fn run(schema: ActixSchema, listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(move || {
        App::new()
            .route("/", web::get().to(get_names))
            .route("/{list_id}", web::get().to(add_name))
            .data(schema.clone())
    })
    .listen(listener)?
    .run();

    Ok(server)
}

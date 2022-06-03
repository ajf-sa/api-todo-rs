pub mod env {
    extern crate dotenv;
    use dotenv::dotenv;
    pub struct Env {
        app_name: String,
        app_port: String,
        jwt_secret: String,
        db_name: String,
        db_user: String,
        db_pass: String,
        db_host: String,
        db_port: String,
    }
    impl Env {
        pub fn new() -> Env {
            dotenv().ok();
            Env {
                app_name: dotenv::var("APP_NAME").unwrap(),
                app_port: dotenv::var("APP_PORT").unwrap(),
                jwt_secret: dotenv::var("JWT_SECRET").unwrap(),
                db_name: dotenv::var("DB_NAME").unwrap(),
                db_user: dotenv::var("DB_USER").unwrap(),
                db_pass: dotenv::var("DB_PASS").unwrap(),
                db_host: dotenv::var("DB_HOST").unwrap(),
                db_port: dotenv::var("DB_PORT").unwrap(),
            }
        }
        pub fn get_jwt_secret(&self) -> String {
            self.jwt_secret.clone()
        }
        pub fn get_db_name(&self) -> String {
            self.db_name.clone()
        }
        pub fn get_db_user(&self) -> String {
            self.db_user.clone()
        }
        pub fn get_db_pass(&self) -> String {
            self.db_pass.clone()
        }
        pub fn get_db_host(&self) -> String {
            self.db_host.clone()
        }
        pub fn get_db_port(&self) -> String {
            self.db_port.clone()
        }
        pub fn get_port(&self) -> String {
            self.app_port.clone()
        }
    }
}

pub mod connect {
    use sqlx::postgres::PgPool;
    use sqlx::{Error, Pool, Postgres};

    #[derive(Debug)]
    pub struct Connect {
        pub db_name: String,
        pub db_user: String,
        pub db_pass: String,
        pub db_host: String,
        pub db_port: String,
    }

    impl Connect {
        pub fn new() -> Connect {
            use crate::utilts::env::Env;
            let env = Env::new();

            Connect {
                db_name: env.get_db_name(),
                db_user: env.get_db_user(),
                db_pass: env.get_db_pass(),
                db_host: env.get_db_host(),
                db_port: env.get_db_port(),
            }
        }
        fn connection_statment(&self) -> String {
            format!(
                "postgres://{}:{}@{}:{}/{}",
                self.db_user, self.db_pass, self.db_host, self.db_port, self.db_name
            )
        }
        pub async fn connect(&self) -> Result<PgPool, Error> {
            let connection_statment = self.connection_statment();
            let pool = PgPool::connect(connection_statment.as_str()).await;
            pool
        }
    }
}

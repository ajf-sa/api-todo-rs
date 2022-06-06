pub mod service {
    use crate::repository::{repository::Repository, user};

    #[derive(Debug, Clone)]
    pub struct Service {
        repo: Repository,
    }
    impl Service {
        pub fn new(repo: Repository) -> Service {
            Service { repo }
        }
        pub async fn set_user(&self, name: String) -> Result<bool, sqlx::Error> {
            self.repo.set_user(name).await?;
            Ok(true)
        }
        pub async fn get_users(&self) -> Result<Vec<user::User>, sqlx::Error> {
            self.repo.get_users().await
        }
    }
}

pub mod handler {
    use actix_web::{web, HttpResponse};

    use crate::{repository::repository::Repository, utilts::env::Env};

    use super::service::Service;

    #[derive(Clone)]
    pub struct ActixSchema {
        pub service: Service,
        pub repository: Repository,
        pub env: Env,
    }

    pub async fn add_name(
        params: web::Path<(String)>,
        schema: actix_web::web::Data<ActixSchema>,
    ) -> Result<impl actix_web::Responder, actix_web::Error> {
        schema.service.set_user(params.to_string()).await;
        Ok(actix_web::HttpResponse::Ok().json(params.to_string()))
    }

    pub async fn get_names(
        schema: actix_web::web::Data<ActixSchema>,
    ) -> Result<impl actix_web::Responder, actix_web::Error> {
        let users = schema.service.get_users().await.unwrap();

        Ok(actix_web::HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&users).unwrap()))
    }
}

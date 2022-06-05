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
    use actix_web::web;

    #[derive(Clone)]
    pub struct ActixSchema {
        pub service: super::service::Service,
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
        let users = match schema.service.get_users().await {
            Ok(users) => users,
            Err(e) => {
                println!("{}", e);
                vec![]
            }
        };

        Ok(actix_web::HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&users).unwrap()))
    }
}

pub mod service {
    use crate::repository::{repository::Repository, user};

    #[derive(Debug)]
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

    #[derive(Debug)]
    pub struct Handler {
        service: super::service::Service,
    }
    impl Handler {
        pub fn new(service: super::service::Service) -> Handler {
            Handler { service }
        }
    }
}

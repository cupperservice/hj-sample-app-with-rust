use crate::domains::repository::user_repository::UserRepository;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub struct Server<T> {
    user_repository: T,
}

impl<T: UserRepository> Server<T> {
    async fn run(&self) -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
        })
            .bind(("127.0.0.1", 8080))?
            .run()
            .await
    }
}

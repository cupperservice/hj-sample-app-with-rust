use crate::domains::repository::user_repository::UserRepository;
use crate::controller::login_controller;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};

pub struct Server<T> {
    pub user_repository: T,
}

impl<T: UserRepository> Server<T> {
    pub async fn run(&self) -> std::io::Result<()> {
        HttpServer::new(|| {
            App::new()
                .service(web::scope("/app")
                    .route("/login", web::post().to(login_controller::login)))
        })
            .bind(("127.0.0.1", 8080))?
            .run()
            .await
    }
}

use actix_web::{web, Responder};
use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct Info {
    user_id: String,
    password: String,
}

pub async fn login(info: web::Json<Info>) -> impl Responder {
    println!("user_id:{}, password:{}", info.user_id, info.password);
    "hello!!!".to_string()
}

mod db;
mod models;
mod api_models;
mod handlers;

use std::sync::Mutex;
use actix_web::{App, HttpServer};
use db::Db;
use actix_web::web::Data;
use crate::handlers::{create_group, create_user, join_group};

#[actix_web::main]
async fn main() {
    let db = Data::new(Mutex::new(Db::default()));
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(create_user)
            .service(create_group)
            .service(join_group)
    })
        .bind(("127.0.0.1", 8000))
        .unwrap()
        .run()
        .await
        .unwrap();
}

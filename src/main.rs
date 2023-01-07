mod db;

use std::sync::Mutex;
use actix_web::{App, HttpServer};
use db::Db;
use actix_web::web::Data;

#[actix_web::main]
async fn main() {
    let db = Data::new(Mutex::new(Db::default()));
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
    })
        .bind(("127.0.0.1", 8000))
        .unwrap()
        .run()
        .await
        .unwrap();
}

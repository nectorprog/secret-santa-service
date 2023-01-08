mod db;
mod models;
mod api_models;
mod handlers;
mod error;

use std::sync::Mutex;
use actix_web::{App, HttpServer};
use db::Db;
use actix_web::web::Data;
use crate::handlers::{appoint_secret_santas, create_group, create_user, join_group, leave_group, make_user_admin, make_user_nonadmin, remove_group, whos_am_i_santa};

#[actix_web::main]
async fn main() {
    let db = Data::new(Mutex::new(Db::default()));
    HttpServer::new(move || {
        App::new()
            .app_data(db.clone())
            .service(create_user)
            .service(create_group)
            .service(join_group)
            .service(make_user_admin)
            .service(make_user_nonadmin)
            .service(leave_group)
            .service(remove_group)
            .service(appoint_secret_santas)
            .service(whos_am_i_santa)
    })
        .bind(("127.0.0.1", 8000))
        .unwrap()
        .run()
        .await
        .unwrap();
}

// #[cfg(test)]
// mod tests {
//     use super::*;
//     use actix_web::{test, web, App};
//
//     fn initialized_db() -> Data<Mutex<Db>> {
//         let mut db = Db::default();
//         db.create_user("first_user".to_string());
//         db.create_user("second_user".to_string());
//
//     }
//
//     #[text]
//     async fn create_user() {
//         let app = test::init_service(
//             App::new()
//         )
//     }
// }
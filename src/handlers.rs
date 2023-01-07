use actix_web::{Responder, HttpResponse, post, get};
use actix_web::web::{Json, Data};
use anyhow::anyhow;
use super::api_models::{AppendUser};
use std::sync::Mutex;
use crate::db::{Db};

#[post("/createuser")]
pub async fn create_user(req: Json<AppendUser>, db: Data<Mutex<Db>>) -> impl Responder {
    let user_name = req.into_inner().name;
    let mut db = db.lock().unwrap();
    HttpResponse::Ok().json(db.append_user(user_name))
}
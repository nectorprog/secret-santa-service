use actix_web::{Responder, HttpResponse, post, get};
use actix_web::web::{Json, Data};
use anyhow::anyhow;
use std::sync::Mutex;
use crate::api_models::{CreateGroup, CreateUser, JoinGroup, MakeUserAdmin, MakeUserNonadmin};
use crate::db::{Db};

#[post("/createuser")]
pub async fn create_user(req: Json<CreateUser>, db: Data<Mutex<Db>>) -> impl Responder {
    let user_name = req.into_inner().name;
    let mut db = db.lock().unwrap();
    HttpResponse::Ok().json(db.create_user(user_name))
}

#[post("/creategroup")]
pub async fn create_group(req: Json<CreateGroup>, db: Data<Mutex<Db>>) -> impl Responder {
    let mut db = db.lock().unwrap();
    HttpResponse::Ok().json(db.create_group(req.initiator_id, req.into_inner().name))
}

#[post("/joingroup")]
pub async fn join_group(req: Json<JoinGroup>, db: Data<Mutex<Db>>) -> impl Responder {
    let mut db = db.lock().unwrap();
    HttpResponse::Ok().json(db.join_group(req.user_id, req.group_id))
}

#[post("/makeuseradmin")]
pub async fn make_user_admin(req: Json<MakeUserAdmin>, db: Data<Mutex<Db>>) -> impl Responder {
    let mut db = db.lock().unwrap();
    HttpResponse::Ok().json(db.make_user_admin(req.initiator_id, req.user_id, req.group_id))
}

#[post("/makeusernonadmin")]
pub async fn make_user_nonadmin(req: Json<MakeUserNonadmin>, db: Data<Mutex<Db>>) -> impl Responder {
    let mut db = db.lock().unwrap();
    HttpResponse::Ok().json(db.make_user_nonadmin(req.user_id, req.group_id))
}
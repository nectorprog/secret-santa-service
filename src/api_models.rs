use serde::{Deserialize};

#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
}

#[derive(Deserialize)]
pub struct CreateGroup {
    pub initiator_id: i32,
    pub name: String,
}

#[derive(Deserialize)]
pub struct JoinGroup {
    pub user_id: i32,
    pub group_id: i32,
}
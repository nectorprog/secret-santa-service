use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct User {
    pub id: i32,
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Group {
    pub id: i32,
    pub name: String,
    pub is_closed: bool,
}

pub struct GroupUser {
    pub user_id: i32,
    pub group_id: i32,
    pub is_admin: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Santa {
    pub santa_id: i32,
    pub user_id: i32,
}
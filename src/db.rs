use crate::models::User;
use serde::Serialize;

#[derive(Default)]
pub struct Db {
    pub users: Vec<User>,
    pub max_user_id: i32,
}

#[derive(Serialize, Default)]
pub struct Response<E: Serialize> {
    pub status: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: E,
}

impl Db {
    pub fn find_user_by_name(&self, name: &str) -> Option<&User> {
        self.users.iter().find(|u| u.name == name)
    }
    pub fn find_user_by_id(&self, id: i32) -> Option<&User> {
        self.users.iter().find(|u| u.id == id)
    }
    pub fn append_user(&mut self, name: String) -> Response<()> {
        match self.find_user_by_name(&name) {
            None => {
                self.max_user_id += 1;
                let max_user_id = self.max_user_id;
                self.users.push(User {id: max_user_id, name});
                Response {
                    status: true,
                    ..Default::default()
                }
            },
            Some(_) => {
                Response {
                    status: false,
                    message: Some("Пользователь с таким именем уже существует".to_string()),
                    ..Default::default()
                }
            }
        }
    }
}
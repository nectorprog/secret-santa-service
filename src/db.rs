use crate::models::{Group, GroupUser, User};
use serde::Serialize;

#[derive(Default)]
pub struct Db {
    users: Vec<User>,
    max_user_id: i32,
    groups: Vec<Group>,
    max_group_id: i32,
    groups_users: Vec<GroupUser>,
}

#[derive(Serialize, Default)]
pub struct Response<E: Serialize> {
    pub status: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: E,
}

#[derive(Serialize)]
pub struct CreatedGroup {
    pub group_id: i32,
}

impl Db {
    pub fn find_user_by_name(&self, name: &str) -> Option<&User> {
        self.users.iter().find(|u| u.name == name)
    }
    pub fn find_user_by_id(&self, id: i32) -> Option<&User> {
        self.users.iter().find(|u| u.id == id)
    }
    pub fn find_group_by_name(&self, name: &str) -> Option<&Group> {
        self.groups.iter().find(|g| g.name == name)
    }
    pub fn find_group_by_id(&self, id: i32) -> Option<&Group> {
        self.groups.iter().find(|g| g.id == id)
    }
    pub fn find_user_group(&self, user_id: i32, group_id: i32) -> Option<&GroupUser> {
        self.groups_users.iter().find(|gu| gu.user_id == user_id && gu.group_id == group_id)
    }
    pub fn create_user(&mut self, name: String) -> Response<()> {
        match self.find_user_by_name(&name) {
            None => {
                self.max_user_id += 1;
                let user_id = self.max_user_id;
                self.users.push(User {id: user_id, name});
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
    pub fn create_group(&mut self, initiator_id: i32, group_name: String) -> Response<Option<CreatedGroup>> {
        if self.find_group_by_name(&group_name).is_some() {
            return Response {
                status: false,
                message: Some("Группа с таким именем уже существует".to_string()),
                ..Default::default()
            }
        };
        if self.find_user_by_id(initiator_id).is_none() {
            return Response {
                status: false,
                message: Some("Не найден пользователь с таким id".to_string()),
                ..Default::default()
            }
        }
        self.max_group_id += 1;
        let group_id = self.max_group_id;
        self.groups.push(Group {
            id: group_id,
            name: group_name,
            is_closed: false,
        });
        self.groups_users.push(GroupUser {
            user_id: initiator_id,
            group_id,
            is_admin: true
        });
        Response {
            status: true,
            extra: Some(CreatedGroup { group_id }),
            ..Default::default()
        }
    }
}
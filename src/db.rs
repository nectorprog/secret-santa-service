use crate::models::{Group, GroupUser, User};
use super::error::Error;

#[derive(Default)]
pub struct Db {
    users: Vec<User>,
    max_user_id: i32,
    groups: Vec<Group>,
    max_group_id: i32,
    groups_users: Vec<GroupUser>,
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
    pub fn create_user(&mut self, name: String) -> Result<i32, Error> {
        match self.find_user_by_name(&name) {
            Some(_) => { Err(Error::UserAlreadyExists(name)) },
            None => {
                self.max_user_id += 1;
                let user_id = self.max_user_id;
                self.users.push(User {id: user_id, name});
                Ok(user_id)
            }
        }
    }
    pub fn create_group(&mut self, initiator_id: i32, name: String) -> Result<i32, Error> {
        if self.find_group_by_name(&name).is_some() {
            return Err(Error::GroupAlreadyExists(name))
        }
        self.find_user_by_id(initiator_id).ok_or(Error::UserNotFound(initiator_id))?;
        self.max_group_id += 1;
        let group_id = self.max_group_id;
        self.groups.push(Group {
            id: group_id,
            name,
            is_closed: false,
        });
        self.groups_users.push(GroupUser {
            user_id: initiator_id,
            group_id,
            is_admin: true
        });
        Ok(group_id)
    }
    pub fn join_group(&mut self, user_id: i32, group_id: i32) -> Result<(), Error> {
        self.find_user_by_id(user_id).ok_or(Error::UserNotFound(user_id))?;
        self.find_group_by_id(group_id).ok_or(Error::GroupNotFound(group_id))?;
        if self.find_user_group(user_id, group_id).is_none() {
            self.groups_users.push(GroupUser {
                user_id, group_id,
                is_admin: false
            })
        };
        Ok(())
    }
    fn check_user_is_admin(&self, user_id: i32, group_id: i32) -> Result<(), Error> {
        self.find_user_by_id(user_id).ok_or(Error::UserNotFound(user_id))?;
        self.find_group_by_id(group_id).ok_or(Error::GroupNotFound(group_id))?;
        match self.find_user_group(user_id, group_id).map(|ug| ug.is_admin) {
            Some(false) => {
                Err(Error::UserIsNotAdmin {user_id, group_id})
            },
            Some(true) => Ok(()),
            None => Err(Error::UserIsNotInGroup {user_id, group_id })
        }
    }
    pub fn make_user_admin(&mut self, initiator_id: i32, user_id: i32, group_id: i32) -> Result<(), Error> {
        self.check_user_is_admin(initiator_id, group_id)?;
        match self.groups_users.iter_mut().find(|gu| gu.user_id == user_id && gu.group_id == group_id) {
            Some(gu) => {
                gu.is_admin = true
            },
            None => {
                self.groups_users.push(GroupUser {
                    user_id, group_id,
                    is_admin: true
                })
            }
        }
        Ok(())
    }
    pub fn has_other_admin(&mut self, user_id: i32, group_id: i32) -> bool {
        self.groups_users
            .iter()
            .any(|gu| gu.group_id == group_id && gu.user_id != user_id && gu.is_admin)
    }
    pub fn make_user_nonadmin(&mut self, user_id: i32, group_id: i32) -> Result<(), Error> {
        self.check_user_is_admin(user_id, group_id)?;
        if !self.has_other_admin(user_id, group_id) {
            return Err(Error::NoOtherAdminsInGroup {user_id, group_id})
        }
        if let Some(gu) =  self.groups_users.iter_mut().find(|gu| gu.user_id == user_id && gu.group_id == group_id) {
            gu.is_admin = false;
        }
        Ok(())
    }
    pub fn leave_group(&mut self, user_id: i32, group_id: i32) -> Result<(), Error> {
        if !self.has_other_admin(user_id, group_id) {
            return Err(Error::NoOtherAdminsInGroup { user_id, group_id })
        }
        match self.find_group_by_id(group_id).map(|g| g.is_closed) {
            Some(true) => return Err(Error::GroupIsClosed(group_id)),
            None => return Err(Error::GroupNotFound(group_id)),
            _ => ()
        };
        let index = self.groups_users
            .iter()
            .position(|gu| gu.user_id == user_id && gu.group_id == group_id);
        match index {
            Some(i) => {
                if self.groups_users[i].is_admin {
                    self.groups_users.swap_remove(i);
                    Ok(())
                } else {
                    Err(Error::UserIsNotAdmin {user_id, group_id})
                }
            },
            None => Ok(())
        }
    }
    pub fn remove_group(&mut self, initiator_id: i32, group_id: i32) -> Result<(), Error> {
        self.check_user_is_admin(initiator_id, group_id)?;
        self.groups_users.retain(|gu| gu.group_id != group_id);
        self.groups.retain(|g| g.id != group_id);
        Ok(())
    }
}
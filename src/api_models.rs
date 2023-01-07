use serde::{Deserialize, Serialize};
use crate::error::Error;

#[derive(Serialize, Default)]
pub struct Response<E: Serialize> {
    pub status: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(flatten)]
    pub extra: Option<E>,
}

impl<E: Serialize> From<Error> for Response<E> {
    fn from(e: Error) -> Self {
        Self {
            status: false,
            message: Some(format!("{}", e)),
            extra: None
        }
    }
}

impl<E: Serialize> From<Result<E, Error>> for Response<E> {
    fn from(res: Result<E, Error>) -> Self {
        match res {
            Ok(extra) => Response::success(extra),
            Err(e) => Response::from(e)
        }
    }
}

impl<E: Serialize> Response<E> {
    pub fn success(extra: E) -> Self {
        Self {
            status: true,
            message: None,
            extra: Some(extra)
        }
    }
}

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

#[derive(Deserialize)]
pub struct MakeUserAdmin {
    pub initiator_id: i32,
    pub user_id: i32,
    pub group_id: i32,
}

#[derive(Deserialize)]
pub struct MakeUserNonadmin {
    pub user_id: i32,
    pub group_id: i32,
}

#[derive(Deserialize)]
pub struct LeaveGroup {
    pub user_id: i32,
    pub group_id: i32,
}

#[derive(Deserialize)]
pub struct RemoveGroup {
    pub initiator_id: i32,
    pub group_id: i32,
}
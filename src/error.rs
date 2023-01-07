#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Пользователь {0} не найден")]
    UserNotFound(i32),
    #[error("Группа {0} не найдена")]
    GroupNotFound(i32),
    #[error("Пользователь {user_id} не является членом группы {group_id}")]
    UserIsNotInGroup{ user_id: i32, group_id: i32 },
    #[error("Пользователь {user_id} не является администратором группы {group_id}")]
    UserIsNotAdmin {user_id: i32, group_id: i32},
    #[error("Пользователь с именем {0} уже существует")]
    UserAlreadyExists(String),
    #[error("Группа с именем {0} уже существует")]
    GroupAlreadyExists(String),
    #[error("В группе {group_id} нет других администраторов, кроме {user_id}")]
    NoOtherAdminsInGroup{ user_id: i32, group_id: i32},
    #[error("Группа {0} закрыта")]
    GroupIsClosed(i32),
    #[error("Группа {0} еще не закрыта")]
    GroupIsNotClosed(i32),
}
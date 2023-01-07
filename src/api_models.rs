use serde::{Deserialize};

#[derive(Deserialize)]
pub struct AppendUser {
    pub name: String,
}


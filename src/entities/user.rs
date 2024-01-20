use rbatis::crud;

pub const TABLE_USERS: &'static str = "users";

#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Users {
    pub id: String,
    pub username: String,
    pub password: String,
}

crud!(Users {},TABLE_USERS);

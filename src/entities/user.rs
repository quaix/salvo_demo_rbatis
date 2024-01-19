use rbatis::crud;
#[derive(Clone, Debug, serde::Serialize, serde::Deserialize)]
pub struct Users{
    pub id: String,
    pub username: String,
    pub password: String,
}

crud!(Users {});

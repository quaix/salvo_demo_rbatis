use rbatis::{Page, PageRequest};
use uuid::Uuid;

use crate::{
    app_response::AppResult,
    db::DB,
    dtos::user::{
        UserAddRequest, UserLoginRequest, UserLoginResponse, UserResponse, UserUpdateRequest,
    },
    entities::user::Users, middleware::jwt::get_token, utils::rand_utils,
};

pub async fn add_user(req: UserAddRequest) -> AppResult<UserResponse> {
    let db = DB.get().ok_or(anyhow::anyhow!("Database connection failed."))?;
    let user = Users {
        id: Uuid::new_v4().to_string(),
        username: req.username.clone(),
        password: rand_utils::hash_password(req.password).await?,
    };
    Users::insert(db, &user).await?;

    Ok(UserResponse {
        id: user.id,
        username: user.username,
    })
}

pub async fn login(req: UserLoginRequest) -> AppResult<UserLoginResponse> {
    let db = DB.get().ok_or(anyhow::anyhow!("Database connection failed."))?;
    let user = Users::select_by_column(db, "username", &req.username).await?;
    if user.len() == 0 {
        return Err(anyhow::anyhow!("User does not exist.").into());
    }
    if rand_utils::verify_password(req.password, user[0].password.clone()).await.is_err() {
        return Err(anyhow::anyhow!("Incorrect password.").into());
    }
    let (token, exp) = get_token(user[0].username.clone(), user[0].id.clone())?;
    let res = UserLoginResponse {
        id: user[0].id.clone(),
        username: user[0].username.clone(),
        token,
        exp,
    };
    Ok(res)
}

pub async fn update_user(req: UserUpdateRequest) -> AppResult<UserResponse> {
    let db = DB.get().ok_or(anyhow::anyhow!("Database connection failed."))?;
    let users = Users::select_by_column(db, "id", &req.id).await?;
    if users.len() == 0 {
        return Err(anyhow::anyhow!("User does not exist.").into());
    }
    let user = Users {
        id: users[0].clone().id,
        username: users[0].clone().username,
        password: rand_utils::hash_password(req.password).await?,
    };
    Users::update_by_column(db, &user, "id").await?;
    Ok(UserResponse {
        id: users[0].id.clone(),
        username: users[0].username.clone(),
    })
}

pub async fn delete_user(req: String) -> AppResult<()> {
    let db = DB.get().ok_or(anyhow::anyhow!("Database connection failed."))?;
    Users::delete_by_column(db, "id", &req).await?;
    Ok(())
}

pub async fn users() -> AppResult<Vec<UserResponse>> {
    let db = DB.get().ok_or(anyhow::anyhow!("Database connection failed."))?;
    let users = Users::select_all(db).await?;

    let res = users
        .into_iter()
        .map(|user| UserResponse {
            id: user.id,
            username: user.username,
        })
        .collect::<Vec<_>>();
    Ok(res)
}

pub async fn users_page(page_req: PageRequest) -> AppResult<Page<Users>> {
    let db = DB.get().ok_or(anyhow::anyhow!("Database connection failed."))?;
    let users = Users::select_page(db, &page_req).await?;
    Ok(users)
}

pub async fn select_page_by_username_like(page_req: PageRequest, username: &str) -> AppResult<Page<Users>> {
    let db = DB.get().ok_or(anyhow::anyhow!("Database connection failed."))?;
    let users = Users::select_page_by_username(db, &page_req, username).await?;

    // let res: Page<Users> = serde_json::from_value(json!(users)).unwrap_or_default();
    Ok(users)
}

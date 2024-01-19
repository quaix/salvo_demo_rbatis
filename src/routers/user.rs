use crate::{
    app_response::ErrorResponseBuilder,
    app_response::{AppResponse, AppResult},
    dtos::user::{
        UserAddRequest, UserLoginRequest, UserLoginResponse, UserResponse, UserUpdateRequest,
    },
    services::user,
};
use salvo::{
    oapi::endpoint,
    http::cookie::Cookie,
    oapi::extract::{JsonBody, PathParam},
    Request, Response,
};
use salvo::Writer;

#[endpoint( tags("comm"),)]
pub async fn post_login(req: JsonBody<UserLoginRequest>, res: &mut Response) {
    let result: AppResult<UserLoginResponse> = user::login(req.0).await;
    match result {
        Ok(data) => {
            let jwt_token = data.token.clone();
            let cookie = Cookie::build(("jwt_token", jwt_token))
                .path("/")
                .http_only(true)
                .build();
            res.add_cookie(cookie);
        }
        Err(e) => ErrorResponseBuilder::with_err(e).into_response(res),
    }
}

#[endpoint(tags("users"))]
pub async fn post_add_user(new_user: JsonBody<UserAddRequest>) -> AppResponse<UserResponse> {
    let result = user::add_user(new_user.0).await;
    AppResponse(result)
}

#[endpoint(  tags("users"),
parameters(
    ("id", description = "user id"),
))]
pub async fn put_update_user(req: &mut Request) -> AppResult<AppResponse<UserResponse>> {
    let req: UserUpdateRequest = req.extract().await?;
    let result = user::update_user(req).await;
    Ok(AppResponse(result))
}

#[endpoint(tags("users"))]
pub async fn delete_user(id: PathParam<String>) -> AppResponse<()> {
    let result = user::delete_user(id.0).await;
    AppResponse(result)
}

#[endpoint(tags("users"))]
pub async fn get_users() -> AppResponse<Vec<UserResponse>> {
    let result = user::users().await;
    AppResponse(result)
}

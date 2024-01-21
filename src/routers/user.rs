use rbatis::{DEFAULT_PAGE_SIZE, Page, PageRequest};
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
use salvo::oapi::extract::QueryParam;
use salvo::prelude::Json;
use salvo::Writer;
use crate::entities::user::Users;

#[endpoint(tags("comm"), )]
pub async fn endpoint_post_login(req: JsonBody<UserLoginRequest>, res: &mut Response) {
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
pub async fn endpoint_post_add_user(new_user: JsonBody<UserAddRequest>) -> AppResponse<UserResponse> {
    let result = user::add_user(new_user.0).await;
    AppResponse(result)
}

#[endpoint(tags("users"))]
pub async fn endpoint_put_update_user(update_user: JsonBody<UserUpdateRequest>) -> AppResult<AppResponse<UserResponse>> {
    // let req: UserUpdateRequest = req.extract().await?;
    let result = user::update_user(update_user.0).await;
    Ok(AppResponse(result))
}

#[endpoint(tags("users"))]
pub async fn endpoint_delete_user(id: PathParam<String>) -> AppResponse<()> {
    let result = user::delete_user(id.0).await;
    AppResponse(result)
}

#[endpoint(tags("users"))]
pub async fn endpoint_get_users() -> AppResponse<Vec<UserResponse>> {
    let result = user::users().await;
    AppResponse(result)
}

#[endpoint(tags("users"))]
pub async fn endpoint_get_users_page(page_no: QueryParam<u64, false>, page_size: QueryParam<u64, false>, do_count: QueryParam<bool, false>) -> AppResponse<Page<Users>> {
    let mut page_req = PageRequest::new(page_no.into_inner().unwrap_or(1), page_size.into_inner().unwrap_or(DEFAULT_PAGE_SIZE));
    page_req.do_count = do_count.into_inner().unwrap_or(true);
    let result = user::users_page(page_req).await;
    AppResponse(result)
}

#[endpoint(tags("users"))]
pub async fn endpoint_select_page_by_username_like(page_no: QueryParam<u64, false>, page_size: QueryParam<u64, false>, do_count: QueryParam<bool, false>, username: QueryParam<&str, false>) -> AppResponse<Page<Users>> {
    let mut page_req = PageRequest::new(page_no.into_inner().unwrap_or(1), page_size.into_inner().unwrap_or(DEFAULT_PAGE_SIZE));
    page_req.do_count = do_count.into_inner().unwrap_or(true);
    let result = user::select_page_by_username_like(page_req, username.into_inner().unwrap_or_default()).await;
    AppResponse(result)
}


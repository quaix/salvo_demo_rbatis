use crate::middleware::{cors::cors_middleware, jwt::jwt_middleware};
use salvo::{
    prelude::{CatchPanic, Logger, OpenApi, SwaggerUi},
    Router,
};

use self::{
    demo::hello,
    user::{
        endpoint_delete_user,
        endpoint_get_users,
        endpoint_get_users_page,
        endpoint_post_add_user,
        endpoint_post_login,
        endpoint_put_update_user,
        endpoint_select_page_by_username_like,
    },
};

pub mod demo;
pub mod user;
mod static_routers;

pub fn router() -> Router {
    let mut no_auth_routers = vec![
        Router::with_path("/api/login").post(endpoint_post_login),
    ];

    let _cors_handler = cors_middleware();

    let mut need_auth_routers = vec![
        Router::with_path("/api/users")
            // .get(get_users)
            .get(endpoint_get_users_page)
            .push(Router::with_path("page_by_username").get(endpoint_select_page_by_username_like))
            .post(endpoint_post_add_user)
            .put(endpoint_put_update_user)
            .push(
                Router::with_path("<id>")
                    .delete(endpoint_delete_user),
            ),
    ];

    let router = Router::new()
        //.hoop(_cors_handler)
        .hoop(Logger::new())
        .hoop(CatchPanic::new())
        .get(hello)
        .append(&mut no_auth_routers)
        .push(
            Router::new()
                .append(&mut need_auth_routers)
                .hoop(jwt_middleware()),
        );
    let doc = OpenApi::new("salvo web api", "0.0.1").merge_router(&router);
    router
        .push(doc.into_router("/api-doc/openapi.json"))
        .push(SwaggerUi::new("/api-doc/openapi.json").into_router("swagger-ui"))
}

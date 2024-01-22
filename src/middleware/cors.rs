use salvo::cors::{AllowHeaders, AllowMethods, AllowOrigin, Cors, CorsHandler};
use tracing::log;
use crate::config::CFG;

pub fn cors_middleware() -> CorsHandler {
    log::info!("------cors middleware cors_allow_origin:{:?}",&CFG.server.cors_allow_origin);
    let cors = Cors::new();
    for origin in &CFG.server.cors_allow_origin {
        if origin.contains("*") {
            cors.clone().allow_origin(AllowOrigin::any());
        } else {
            cors.clone().allow_origin(origin);
        }
        log::info!("------cors middleware allow_origin:{:?}",origin)
    }

    let cors_handler = cors
        .allow_methods(AllowMethods::any())
        .allow_headers(AllowHeaders::any())
        .into_handler();
    log::info!("------cors middleware end");
    cors_handler
}

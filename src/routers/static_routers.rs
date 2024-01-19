use rust_embed::RustEmbed;
use salvo::{Router, serve_static::static_embed, oapi::endpoint, Response, http::ResBody, hyper::body::Bytes};
#[derive(RustEmbed)]
#[folder = "assets"]
struct Assets;

#[allow(dead_code)]
pub fn create_static_routers() -> Vec<Router> {
    let static_router = Router::with_path("assets/<**path>").get(static_embed::<Assets>());
    let icon_router = Router::with_path("favicon.ico").get(get_icon);
    vec![static_router,icon_router]
}

#[endpoint(tags("comm"))]
pub async fn get_icon(res: &mut Response) {
    let icon = Assets::get("favicon.ico").unwrap();
    res.body(ResBody::Once(Bytes::from(icon.data.to_vec())));
}

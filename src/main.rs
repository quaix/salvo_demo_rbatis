use crate::db::init_db_conn;
use tokio::signal;
use tracing::{info, log};
use crate::middleware::handle_404::handle_404;
use crate::routers::router;
use config::{CERT_KEY, CFG};
use salvo::server::ServerHandle;
use salvo::catcher::Catcher;
use salvo::conn::rustls::{Keycert, RustlsConfig};
use salvo::prelude::*;
use tracing_subscriber::{EnvFilter, fmt};

mod app_error;
mod app_response;
mod config;
mod db;
mod dtos;
mod services;
mod utils;
mod entities;
mod middleware;
mod routers;
mod unit_tests;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    if std::env::var("RUST_LOG").is_err() {
        std::env::set_var("RUST_LOG", "info");
    }

    init_tracing_subscriber();

    log::debug!(
        "------environment:{:?}",
        std::env::vars_os().collect::<Vec<_>>()
    );

    init_db_conn().await;
    let router = router();
    let service: Service = router.into();
    let service = service.catcher(Catcher::default().hoop(handle_404));
    println!("🌪️ {} is starting ", &CFG.server.name);
    println!("🔄 listen on {}", &CFG.server.address);

    match CFG.server.ssl {
        true => {
            println!(
                "📖 Open API Page: https://{}/swagger-ui",
                &CFG.server.address.replace("0.0.0.0", "127.0.0.1")
            );
            let config = RustlsConfig::new(
                Keycert::new()
                    .cert(CERT_KEY.cert.clone())
                    .key(CERT_KEY.key.clone()),
            );
            let acceptor = TcpListener::new(&CFG.server.address)
                .rustls(config)
                .bind()
                .await;
            let server = Server::new(acceptor);
            let handle = server.handle();
            tokio::spawn(shutdown_signal(handle));
            server.serve(service).await;
        }
        false => {
            println!(
                "📖 Open API Page: http://{}/swagger-ui",
                &CFG.server.address.replace("0.0.0.0", "127.0.0.1")
            );
            let acceptor = TcpListener::new(&CFG.server.address).bind().await;
            let server = Server::new(acceptor);
            let handle = server.handle();
            tokio::spawn(shutdown_signal(handle));
            server.serve(service).await;
        }
    }
}


fn init_tracing_subscriber() {
    // 从环境变量中获取日志级别，默认为 info
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));

    //https://users.rust-lang.org/t/best-way-to-log-with-json/83385
    // https://github.com/tokio-rs/tracing/pull/1772
    let style = std::env::var("RUST_LOG_STYLE").unwrap_or_else(|_| "auto".into());
    let tracing_format = std::env::var("TRACING_FORMAT").unwrap_or_else(|_| "ansi".into());

    let subscriber_builder = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_timer(fmt::time::time())
        .with_ansi(style.to_lowercase() != "never");

    match tracing_format.as_str() {
        "json" => subscriber_builder.json().init(),
        _ => subscriber_builder.init(),
    };
}

async fn shutdown_signal(handle: ServerHandle) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
        let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => info!("ctrl_c signal received"),
        _ = terminate => info!("terminate signal received"),
    }
    handle.stop_graceful(std::time::Duration::from_secs(60));
}

#[cfg(test)]
mod tests {
    use salvo::prelude::*;
    use salvo::test::{ResponseExt, TestClient};

    use crate::config::CFG;

    #[tokio::test]
    async fn test_hello_world() {
        let service = Service::new(super::router());

        let content = TestClient::get(format!(
            "http://{}",
            &CFG.server.address.replace("0.0.0.0", "127.0.0.1")
        ))
            .send(&service)
            .await
            .take_string()
            .await
            .unwrap();
        assert_eq!(content, "Hello World from salvo");
    }
}

use crate::db::init_db_conn;
use tokio::signal;
use tracing::info;
use crate::middleware::handle_404::handle_404;
use crate::routers::router;
use config::{CERT_KEY, CFG};
use salvo::server::ServerHandle;
use salvo::catcher::Catcher;
use salvo::conn::rustls::{Keycert, RustlsConfig};
use salvo::prelude::*;
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
    //At the same time, logs are only output to the terminal or file
    let _guard = clia_tracing_config::build()
        .filter_level(&CFG.log.filter_level)
        .with_ansi(CFG.log.with_ansi)
        .to_stdout(CFG.log.to_stdout)
        .directory(&CFG.log.directory)
        .file_name(&CFG.log.file_name)
        .rolling(&CFG.log.rolling)
        .init();
    tracing::info!("log level: {}", &CFG.log.filter_level);

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

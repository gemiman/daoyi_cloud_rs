use daoyi_framework::{dy_config, hoops};
use salvo::catcher::Catcher;
use salvo::prelude::*;
use salvo::server::ServerHandle;
use tokio::signal;

mod routers;

#[tokio::main]
async fn main() {
    dy_config::init().await;

    let service = Service::new(routers::root())
        .catcher(Catcher::default().hoop(hoops::error_404))
        .hoop(hoops::cors_hoop());
    let listen_addr = dy_config::get().get_listen_addr();
    tracing::info!("🔄 在以下位置监听 {}", listen_addr);
    tracing::info!(
        "📖 Open API 页面: http://{}/scalar",
        listen_addr.replace("0.0.0.0", "127.0.0.1")
    );
    tracing::info!(
        "🔑 Login Page: http://{}/login",
        listen_addr.replace("0.0.0.0", "127.0.0.1")
    );
    let acceptor = TcpListener::new(listen_addr).bind().await;
    let server = Server::new(acceptor);
    tokio::spawn(shutdown_signal(server.handle()));
    server.serve(service).await;
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
        _ = ctrl_c => tracing::info!("ctrl_c signal received"),
        _ = terminate => tracing::info!("terminate signal received"),
    }
    handle.stop_graceful(std::time::Duration::from_secs(60));
}

#[cfg(test)]
mod tests {
    use daoyi_framework::dy_config;
    use salvo::prelude::*;
    use salvo::test::{ResponseExt, TestClient};

    #[tokio::test]
    async fn test_hello_world() {
        dy_config::init().await;

        let service = Service::new(crate::routers::root());

        let content = TestClient::get(format!(
            "http://{}",
            dy_config::get()
                .get_listen_addr()
                .replace("0.0.0.0", "127.0.0.1")
        ))
        .send(&service)
        .await
        .take_string()
        .await
        .unwrap();
        assert_eq!(content, "Hello World from salvo");
    }
}

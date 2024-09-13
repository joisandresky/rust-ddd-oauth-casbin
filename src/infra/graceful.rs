use tokio::signal;
use tracing::debug;

pub async fn shutdown_signal() {
    let svc = "API Campusflow";

    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install CTRL+C signal handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install terminate signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {
            debug!("Got CTRL+C signal on {}", svc);
        }
        _ = terminate => {
            debug!("Got terminate signal on {}", svc);
        }
    }
}

use crate::problem::BizProblem;
use axum::{
    Router,
    body::Body,
    http::{Response, StatusCode},
    response::IntoResponse,
};
use std::{any::Any, net::SocketAddr};
use tokio::{net::TcpListener, signal};

pub async fn serve(router: Router, addr: SocketAddr) {
    let _ = axum::serve(TcpListener::bind(addr).await.unwrap(), router)
        .with_graceful_shutdown(shutdown())
        .await;
}

pub async fn shutdown() {
    let ctrl_c = async {
        signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
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
        _ = ctrl_c => {},
        _ = terminate => {},
    }
}

pub async fn handler_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "How ! 404.")
}

pub fn handler_panic(err: Box<dyn Any + Send + 'static>) -> Response<Body> {
    let msg = if let Some(s) = err.downcast_ref::<String>() {
        s.clone()
    } else if let Some(s) = err.downcast_ref::<&str>() {
        s.to_string()
    } else {
        "hava no panic message".to_string()
    };
    BizProblem::Unknow.with_msg(msg).into_response()
}

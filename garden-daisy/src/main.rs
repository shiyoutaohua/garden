#![allow(dead_code)]
#![allow(async_fn_in_trait)]
use crate::router::BaseRouter;
use axum::{Router, extract::DefaultBodyLimit, http::StatusCode, middleware};
use garden_core::{
    middleware::request_id::ensure_request_id,
    util::app::{self, handler_panic},
};
use rustls::crypto::aws_lc_rs;
use std::{
    net::{Ipv4Addr, SocketAddr},
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};
use tower::ServiceBuilder;
use tower_http::{
    catch_panic::CatchPanicLayer,
    compression::CompressionLayer,
    cors::{AllowCredentials, AllowHeaders, AllowMethods, AllowOrigin, CorsLayer},
    request_id::PropagateRequestIdLayer,
    timeout::TimeoutLayer,
    trace::TraceLayer,
};
use tracing::{Level, Span, info};

pub mod handler;
pub mod router;

fn main() {
    aws_lc_rs::default_provider()
        .install_default()
        .expect("failed to install rustls aws-lc-rs crypto provider");
    tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .with_thread_names(true)
        .with_line_number(true)
        .with_timer(tracing_subscriber::fmt::time::OffsetTime::local_rfc_3339().expect("can't get local offset"))
        .init();
    let rt = tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .thread_name_fn(|| {
            static TOKIO_WORKER_ID: AtomicU64 = AtomicU64::new(0);
            let id = TOKIO_WORKER_ID.fetch_add(1, Ordering::Relaxed);
            format!("tokio-worker-{id}")
        })
        .build()
        .expect("can't create tokio runtime");
    rt.block_on(init());
}

pub async fn init() {
    let cors_layer = CorsLayer::new()
        .allow_origin(AllowOrigin::mirror_request())
        .allow_methods(AllowMethods::mirror_request())
        .allow_headers(AllowHeaders::mirror_request())
        .allow_credentials(AllowCredentials::yes())
        .max_age(Duration::from_secs(3600));
    // router
    let app_router = Router::new()
        .merge(BaseRouter::routes())
        .fallback(app::handler_404)
        .layer(
            ServiceBuilder::new()
                .layer(CatchPanicLayer::custom(handler_panic))
                .layer(middleware::from_fn(ensure_request_id))
                .layer(TraceLayer::new_for_http().make_span_with(|_: &axum::http::Request<_>| Span::current()))
                .layer(cors_layer.clone())
                .layer(PropagateRequestIdLayer::x_request_id())
                .layer(TimeoutLayer::with_status_code(
                    StatusCode::REQUEST_TIMEOUT,
                    Duration::from_secs(30),
                ))
                .layer(DefaultBodyLimit::max(30 * 1024 * 1024))
                .layer(CompressionLayer::new()),
        );
    let app_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), 8080);
    info!("app listening on {}", app_addr);
    tokio::join!(app::serve(app_router, app_addr),);
}

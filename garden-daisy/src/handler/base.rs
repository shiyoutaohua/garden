use axum::{
    Json,
    extract::{Path as PathVar, Query},
    http::HeaderMap,
    response::IntoResponse,
};
use garden_core::model::result::{base::BizResult, metrics::AppMetrics};
use std::collections::HashMap;
use tracing::debug;

pub(crate) struct BaseHandler;
impl BaseHandler {
    pub(crate) async fn greet() -> BizResult<String> {
        debug!("f[greet] begin");
        let app_name = "daisy";
        let reply = format!("Hey from {}", app_name);
        debug!("f[greet] end");
        BizResult::ok(reply)
    }

    pub(crate) async fn healthz() -> impl IntoResponse {
        let metrics = AppMetrics {
            uptime: 1,
            start_ts: 1,
            start_iso: String::new(),
        };
        Json(metrics)
    }

    pub(crate) async fn path(PathVar(key): PathVar<String>) -> impl IntoResponse {
        key
    }

    pub(crate) async fn query(Query(map): Query<HashMap<String, String>>) -> impl IntoResponse {
        format!("{:?}", map)
    }

    pub(crate) async fn headers(header_map: HeaderMap) -> impl IntoResponse {
        BizResult::ok(format!("{:?}", header_map))
    }

    pub(crate) async fn post_text(body: String) -> impl IntoResponse {
        BizResult::ok(body)
    }
}

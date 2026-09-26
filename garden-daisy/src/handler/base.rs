use axum::{
    Json,
    extract::{Path as PathVar, Query},
    response::IntoResponse,
};
use garden_core::{model::result::metrics::AppMetrics, problem::BizProblem};
use std::collections::HashMap;
use tracing::debug;

pub(crate) struct BaseHandler;
impl BaseHandler {
    pub(crate) async fn greet() -> impl IntoResponse {
        debug!("f[greet] begin");
        let app_name = crate::cfg::app::name();
        let reply = format!("Hey from {}", app_name);
        debug!("f[greet] end");
        reply
    }

    pub(crate) async fn healthz() -> impl IntoResponse {
        let metrics = AppMetrics {
            uptime: 1,
            start_ts: 1,
            start_iso: String::new(),
        };
        Json(metrics)
    }

    pub(crate) async fn problem() -> impl IntoResponse {
        BizProblem::Unknow
    }

    pub(crate) async fn path(PathVar(key): PathVar<String>) -> impl IntoResponse {
        key
    }

    pub(crate) async fn query(Query(map): Query<HashMap<String, String>>) -> impl IntoResponse {
        format!("{:?}", map)
    }
}

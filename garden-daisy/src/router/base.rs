use crate::handler::BaseHandler;
use axum::{
    Router,
    routing::{get, post},
};

pub(crate) struct BaseRouter;
impl BaseRouter {
    pub(crate) fn routes() -> Router {
        Router::new()
            .route("/", get(BaseHandler::greet))
            .route("/healthz", get(BaseHandler::healthz))
            .route("/problem", get(BaseHandler::problem))
            .route("/path/{key}", get(BaseHandler::path))
            .route("/query", get(BaseHandler::query))
            .route("/headers", get(BaseHandler::headers))
            .route("/post-text", post(BaseHandler::post_text))
    }
}

use crate::handler::BaseHandler;
use axum::{Router, routing::get};

pub(crate) struct BaseRouter;
impl BaseRouter {
    pub(crate) fn routes() -> Router {
        Router::new()
            .route("/", get(BaseHandler::greet))
            .route("/path/{key}", get(BaseHandler::path))
            .route("/query", get(BaseHandler::query))
    }
}

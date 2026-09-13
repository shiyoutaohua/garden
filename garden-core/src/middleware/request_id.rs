use crate::{constant::http::HEADER_REQUEST_ID, problem::biz::BizProblem, util::id::UlidGenerator};
use axum::{http::HeaderValue, middleware::Next, response::Response};
use tracing::{Instrument, debug, info_span};

pub async fn ensure_request_id(mut req: axum::extract::Request, next: Next) -> Result<Response, BizProblem> {
    // ensure
    let reqid = req
        .headers()
        .get(HEADER_REQUEST_ID)
        .and_then(|v| v.to_str().ok())
        .filter(|s| is_valid(s))
        .map(|s| s.to_owned())
        .unwrap_or_else(UlidGenerator::next);
    req.headers_mut().insert(
        HEADER_REQUEST_ID,
        HeaderValue::from_str(&reqid).expect(format!("{HEADER_REQUEST_ID} must be valid http header value").as_str()),
    );
    // make request span
    let span = info_span!(
        "request",
        method = %req.method(),
        uri = %req.uri(),
        version = ?req.version(),
        rayid = %reqid
    );
    async move {
        debug!("ensure {HEADER_REQUEST_ID} done");
        // call next service
        let response = next.run(req).await;
        // do something with `response`...
        Ok(response)
    }
    .instrument(span)
    .await
}

fn is_valid(s: &str) -> bool {
    const THRESHOLD: usize = 1024;
    !s.is_empty() && s.len() <= THRESHOLD && s.chars().all(|c| c.is_ascii_alphanumeric() || c == '-')
}

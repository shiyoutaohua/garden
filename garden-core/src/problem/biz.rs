//! Inspired by rfc9457
use axum::{
    Json,
    http::{StatusCode, header::CONTENT_TYPE},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BizProblem {
    status: u16,
    kind: Cow<'static, str>,
    msg: Cow<'static, str>,
}

impl BizProblem {
    pub const UNKNOW: Self = Self {
        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        kind: Cow::Borrowed("sys.unknown"),
        msg: Cow::Borrowed("unknown"),
    };

    pub const DATA_SERDE: Self = Self {
        status: StatusCode::BAD_REQUEST.as_u16(),
        kind: Cow::Borrowed("sys.serde"),
        msg: Cow::Borrowed("fail to serde"),
    };

    pub const CONSUL: Self = Self {
        status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
        kind: Cow::Borrowed("sys.consul"),
        msg: Cow::Borrowed("consul operation failed"),
    };

    pub const REDIS: Self = Self {
        status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
        kind: Cow::Borrowed("sys.redis"),
        msg: Cow::Borrowed("redis operation failed"),
    };

    pub const DB: Self = Self {
        status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
        kind: Cow::Borrowed("sys.db"),
        msg: Cow::Borrowed("db operation failed"),
    };

    pub const OPENSEARCH: Self = Self {
        status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
        kind: Cow::Borrowed("sys.opensearch"),
        msg: Cow::Borrowed("opensearch operation failed"),
    };

    pub const NATS: Self = Self {
        status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
        kind: Cow::Borrowed("sys.nats"),
        msg: Cow::Borrowed("nats operation failed"),
    };

    pub const FILEIO: Self = Self {
        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        kind: Cow::Borrowed("sys.io.file"),
        msg: Cow::Borrowed("file i/o operation failed"),
    };

    pub const NETWORK: Self = Self {
        status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
        kind: Cow::Borrowed("sys.network"),
        msg: Cow::Borrowed("network operation failed"),
    };

    pub const RATELIMIT: Self = Self {
        status: StatusCode::TOO_MANY_REQUESTS.as_u16(),
        kind: Cow::Borrowed("sys.rate.limit"),
        msg: Cow::Borrowed("too many request"),
    };

    pub const ILLEGAL_STATE: Self = Self {
        status: StatusCode::CONFLICT.as_u16(),
        kind: Cow::Borrowed("common.state.illegal"),
        msg: Cow::Borrowed("illegal state"),
    };

    pub const ILLEGAL_FORMAT: Self = Self {
        status: StatusCode::BAD_REQUEST.as_u16(),
        kind: Cow::Borrowed("common.format.illegal"),
        msg: Cow::Borrowed("illegal format"),
    };

    pub const ILLEGAL_ARGUMENT: Self = Self {
        status: StatusCode::BAD_REQUEST.as_u16(),
        kind: Cow::Borrowed("common.argument.illegal"),
        msg: Cow::Borrowed("illegal argument"),
    };

    pub const PARAMETER_MISSING: Self = Self {
        status: StatusCode::BAD_REQUEST.as_u16(),
        kind: Cow::Borrowed("common.parameter.missing"),
        msg: Cow::Borrowed("parameter missing"),
    };

    pub const PARAMETER_INVALID: Self = Self {
        status: StatusCode::BAD_REQUEST.as_u16(),
        kind: Cow::Borrowed("common.parameter.invalid"),
        msg: Cow::Borrowed("parameter invalid"),
    };

    pub const PASSWORD_PATTERN: Self = Self {
        status: StatusCode::BAD_REQUEST.as_u16(),
        kind: Cow::Borrowed("auth.password.invalid"),
        msg: Cow::Borrowed("password pattern invalid"),
    };

    pub const PASSWORD_HASH: Self = Self {
        status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
        kind: Cow::Borrowed("auth.password.hash"),
        msg: Cow::Borrowed("fail to hash password"),
    };

    pub const PASSWORD_VERIFY: Self = Self {
        status: StatusCode::UNAUTHORIZED.as_u16(),
        kind: Cow::Borrowed("auth.password.verify"),
        msg: Cow::Borrowed("fail to verify password"),
    };

    pub const APIKEY_MISSING: Self = Self {
        status: StatusCode::UNAUTHORIZED.as_u16(),
        kind: Cow::Borrowed("auth.apikey.missing"),
        msg: Cow::Borrowed("apikey missing"),
    };

    pub const APIKEY_INVALID: Self = Self {
        status: StatusCode::UNAUTHORIZED.as_u16(),
        kind: Cow::Borrowed("auth.apikey.invalid"),
        msg: Cow::Borrowed("apikey invalid"),
    };

    pub const TOKEN_MISSING: Self = Self {
        status: StatusCode::UNAUTHORIZED.as_u16(),
        kind: Cow::Borrowed("auth.token.missing"),
        msg: Cow::Borrowed("token missing"),
    };

    pub const TOKEN_INVALID: Self = Self {
        status: StatusCode::UNAUTHORIZED.as_u16(),
        kind: Cow::Borrowed("auth.token.invalid"),
        msg: Cow::Borrowed("token invalid"),
    };

    pub const USER_NOT_EXIST: Self = Self {
        status: StatusCode::NOT_FOUND.as_u16(),
        kind: Cow::Borrowed("user.not-exist"),
        msg: Cow::Borrowed("user not exist"),
    };

    pub fn with_msg(self, msg: impl Into<String>) -> Self {
        Self {
            msg: Cow::Owned(msg.into()),
            ..self
        }
    }

    pub fn concat_msg(self, suffix: impl Into<String>) -> Self {
        Self {
            msg: Cow::Owned(format!("{}; {}", self.msg, suffix.into())),
            ..self
        }
    }
}

impl IntoResponse for BizProblem {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, [(CONTENT_TYPE, "application/problem+json")], Json(self)).into_response()
    }
}

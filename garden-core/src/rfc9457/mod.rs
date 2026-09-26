//!  Rfc9457
use axum::{
    Json,
    http::{StatusCode, header::CONTENT_TYPE},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProblemBody {
    status: u16,
    kind: Cow<'static, str>,
    msg: Cow<'static, str>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Rfc9457 {
    Unknow,
    DataSerde,
    Consul,
    Redis,
    Db,
    Opensearch,
    Nats,
    Fileio,
    Network,
    Ratelimit,
    IllegalState,
    IllegalFormat,
    IllegalArgument,
    ParameterMissing,
    ParameterInvalid,
    PasswordPattern,
    PasswordHash,
    PasswordVerify,
    ApikeyMissing,
    ApikeyInvalid,
    TokenMissing,
    TokenInvalid,
    UserNotExist,
}

impl Rfc9457 {
    pub fn body(&self) -> ProblemBody {
        match self {
            Rfc9457::Unknow => ProblemBody {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                kind: Cow::Borrowed("sys.unknown"),
                msg: Cow::Borrowed("unknown"),
            },
            Rfc9457::DataSerde => ProblemBody {
                status: StatusCode::BAD_REQUEST.as_u16(),
                kind: Cow::Borrowed("sys.serde"),
                msg: Cow::Borrowed("fail to serde"),
            },
            Rfc9457::Consul => ProblemBody {
                status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
                kind: Cow::Borrowed("sys.consul"),
                msg: Cow::Borrowed("consul operation failed"),
            },
            Rfc9457::Redis => ProblemBody {
                status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
                kind: Cow::Borrowed("sys.redis"),
                msg: Cow::Borrowed("redis operation failed"),
            },
            Rfc9457::Db => ProblemBody {
                status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
                kind: Cow::Borrowed("sys.db"),
                msg: Cow::Borrowed("db operation failed"),
            },
            Rfc9457::Opensearch => ProblemBody {
                status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
                kind: Cow::Borrowed("sys.opensearch"),
                msg: Cow::Borrowed("opensearch operation failed"),
            },
            Rfc9457::Nats => ProblemBody {
                status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
                kind: Cow::Borrowed("sys.nats"),
                msg: Cow::Borrowed("nats operation failed"),
            },
            Rfc9457::Fileio => ProblemBody {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                kind: Cow::Borrowed("sys.io.file"),
                msg: Cow::Borrowed("file i/o operation failed"),
            },
            Rfc9457::Network => ProblemBody {
                status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
                kind: Cow::Borrowed("sys.network"),
                msg: Cow::Borrowed("network operation failed"),
            },
            Rfc9457::Ratelimit => ProblemBody {
                status: StatusCode::TOO_MANY_REQUESTS.as_u16(),
                kind: Cow::Borrowed("sys.rate.limit"),
                msg: Cow::Borrowed("too many request"),
            },
            Rfc9457::IllegalState => ProblemBody {
                status: StatusCode::CONFLICT.as_u16(),
                kind: Cow::Borrowed("common.state.illegal"),
                msg: Cow::Borrowed("illegal state"),
            },
            Rfc9457::IllegalFormat => ProblemBody {
                status: StatusCode::BAD_REQUEST.as_u16(),
                kind: Cow::Borrowed("common.format.illegal"),
                msg: Cow::Borrowed("illegal format"),
            },
            Rfc9457::IllegalArgument => ProblemBody {
                status: StatusCode::BAD_REQUEST.as_u16(),
                kind: Cow::Borrowed("common.argument.illegal"),
                msg: Cow::Borrowed("illegal argument"),
            },
            Rfc9457::ParameterMissing => ProblemBody {
                status: StatusCode::BAD_REQUEST.as_u16(),
                kind: Cow::Borrowed("common.parameter.missing"),
                msg: Cow::Borrowed("parameter missing"),
            },
            Rfc9457::ParameterInvalid => ProblemBody {
                status: StatusCode::BAD_REQUEST.as_u16(),
                kind: Cow::Borrowed("common.parameter.invalid"),
                msg: Cow::Borrowed("parameter invalid"),
            },
            Rfc9457::PasswordPattern => ProblemBody {
                status: StatusCode::BAD_REQUEST.as_u16(),
                kind: Cow::Borrowed("auth.password.invalid"),
                msg: Cow::Borrowed("password pattern invalid"),
            },
            Rfc9457::PasswordHash => ProblemBody {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                kind: Cow::Borrowed("auth.password.hash"),
                msg: Cow::Borrowed("fail to hash password"),
            },
            Rfc9457::PasswordVerify => ProblemBody {
                status: StatusCode::UNAUTHORIZED.as_u16(),
                kind: Cow::Borrowed("auth.password.verify"),
                msg: Cow::Borrowed("fail to verify password"),
            },
            Rfc9457::ApikeyMissing => ProblemBody {
                status: StatusCode::UNAUTHORIZED.as_u16(),
                kind: Cow::Borrowed("auth.apikey.missing"),
                msg: Cow::Borrowed("apikey missing"),
            },
            Rfc9457::ApikeyInvalid => ProblemBody {
                status: StatusCode::UNAUTHORIZED.as_u16(),
                kind: Cow::Borrowed("auth.apikey.invalid"),
                msg: Cow::Borrowed("apikey invalid"),
            },
            Rfc9457::TokenMissing => ProblemBody {
                status: StatusCode::UNAUTHORIZED.as_u16(),
                kind: Cow::Borrowed("auth.token.missing"),
                msg: Cow::Borrowed("token missing"),
            },
            Rfc9457::TokenInvalid => ProblemBody {
                status: StatusCode::UNAUTHORIZED.as_u16(),
                kind: Cow::Borrowed("auth.token.invalid"),
                msg: Cow::Borrowed("token invalid"),
            },
            Rfc9457::UserNotExist => ProblemBody {
                status: StatusCode::NOT_FOUND.as_u16(),
                kind: Cow::Borrowed("user.not-exist"),
                msg: Cow::Borrowed("user not exist"),
            },
        }
    }
}

impl IntoResponse for Rfc9457 {
    fn into_response(self) -> Response {
        let body = self.body();
        let status = StatusCode::from_u16(body.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, [(CONTENT_TYPE, "application/problem+json")], Json(body)).into_response()
    }
}

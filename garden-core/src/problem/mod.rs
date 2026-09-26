//!  Rfc9457
use axum::{
    Json,
    http::{StatusCode, header::CONTENT_TYPE},
    response::{IntoResponse, Response},
};
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
struct ProblemBody {
    status: u16,
    kind: Cow<'static, str>,
    msg: Cow<'static, str>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum BizProblem {
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

impl BizProblem {
    fn body(&self) -> ProblemBody {
        match self {
            BizProblem::Unknow => ProblemBody {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                kind: Cow::Borrowed("sys.unknown"),
                msg: Cow::Borrowed("unknown"),
            },
            BizProblem::DataSerde => ProblemBody {
                status: StatusCode::BAD_REQUEST.as_u16(),
                kind: Cow::Borrowed("sys.serde"),
                msg: Cow::Borrowed("fail to serde"),
            },
            BizProblem::Consul => ProblemBody {
                status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
                kind: Cow::Borrowed("sys.consul"),
                msg: Cow::Borrowed("consul operation failed"),
            },
            BizProblem::Redis => ProblemBody {
                status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
                kind: Cow::Borrowed("sys.redis"),
                msg: Cow::Borrowed("redis operation failed"),
            },
            BizProblem::Db => ProblemBody {
                status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
                kind: Cow::Borrowed("sys.db"),
                msg: Cow::Borrowed("db operation failed"),
            },
            BizProblem::Opensearch => ProblemBody {
                status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
                kind: Cow::Borrowed("sys.opensearch"),
                msg: Cow::Borrowed("opensearch operation failed"),
            },
            BizProblem::Nats => ProblemBody {
                status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
                kind: Cow::Borrowed("sys.nats"),
                msg: Cow::Borrowed("nats operation failed"),
            },
            BizProblem::Fileio => ProblemBody {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                kind: Cow::Borrowed("sys.io.file"),
                msg: Cow::Borrowed("file i/o operation failed"),
            },
            BizProblem::Network => ProblemBody {
                status: StatusCode::SERVICE_UNAVAILABLE.as_u16(),
                kind: Cow::Borrowed("sys.network"),
                msg: Cow::Borrowed("network operation failed"),
            },
            BizProblem::Ratelimit => ProblemBody {
                status: StatusCode::TOO_MANY_REQUESTS.as_u16(),
                kind: Cow::Borrowed("sys.rate.limit"),
                msg: Cow::Borrowed("too many request"),
            },
            BizProblem::IllegalState => ProblemBody {
                status: StatusCode::CONFLICT.as_u16(),
                kind: Cow::Borrowed("common.state.illegal"),
                msg: Cow::Borrowed("illegal state"),
            },
            BizProblem::IllegalFormat => ProblemBody {
                status: StatusCode::BAD_REQUEST.as_u16(),
                kind: Cow::Borrowed("common.format.illegal"),
                msg: Cow::Borrowed("illegal format"),
            },
            BizProblem::IllegalArgument => ProblemBody {
                status: StatusCode::BAD_REQUEST.as_u16(),
                kind: Cow::Borrowed("common.argument.illegal"),
                msg: Cow::Borrowed("illegal argument"),
            },
            BizProblem::ParameterMissing => ProblemBody {
                status: StatusCode::BAD_REQUEST.as_u16(),
                kind: Cow::Borrowed("common.parameter.missing"),
                msg: Cow::Borrowed("parameter missing"),
            },
            BizProblem::ParameterInvalid => ProblemBody {
                status: StatusCode::BAD_REQUEST.as_u16(),
                kind: Cow::Borrowed("common.parameter.invalid"),
                msg: Cow::Borrowed("parameter invalid"),
            },
            BizProblem::PasswordPattern => ProblemBody {
                status: StatusCode::BAD_REQUEST.as_u16(),
                kind: Cow::Borrowed("auth.password.invalid"),
                msg: Cow::Borrowed("password pattern invalid"),
            },
            BizProblem::PasswordHash => ProblemBody {
                status: StatusCode::INTERNAL_SERVER_ERROR.as_u16(),
                kind: Cow::Borrowed("auth.password.hash"),
                msg: Cow::Borrowed("fail to hash password"),
            },
            BizProblem::PasswordVerify => ProblemBody {
                status: StatusCode::UNAUTHORIZED.as_u16(),
                kind: Cow::Borrowed("auth.password.verify"),
                msg: Cow::Borrowed("fail to verify password"),
            },
            BizProblem::ApikeyMissing => ProblemBody {
                status: StatusCode::UNAUTHORIZED.as_u16(),
                kind: Cow::Borrowed("auth.apikey.missing"),
                msg: Cow::Borrowed("apikey missing"),
            },
            BizProblem::ApikeyInvalid => ProblemBody {
                status: StatusCode::UNAUTHORIZED.as_u16(),
                kind: Cow::Borrowed("auth.apikey.invalid"),
                msg: Cow::Borrowed("apikey invalid"),
            },
            BizProblem::TokenMissing => ProblemBody {
                status: StatusCode::UNAUTHORIZED.as_u16(),
                kind: Cow::Borrowed("auth.token.missing"),
                msg: Cow::Borrowed("token missing"),
            },
            BizProblem::TokenInvalid => ProblemBody {
                status: StatusCode::UNAUTHORIZED.as_u16(),
                kind: Cow::Borrowed("auth.token.invalid"),
                msg: Cow::Borrowed("token invalid"),
            },
            BizProblem::UserNotExist => ProblemBody {
                status: StatusCode::NOT_FOUND.as_u16(),
                kind: Cow::Borrowed("user.not-exist"),
                msg: Cow::Borrowed("user not exist"),
            },
        }
    }

    pub fn with_msg(self, msg: impl Into<String>) -> impl IntoResponse {
        ProblemBody {
            msg: Cow::Owned(msg.into()),
            ..self.body()
        }
    }

    pub fn concat_msg(self, suffix: impl Into<String>) -> impl IntoResponse {
        let mut pb = self.body();
        pb.msg = Cow::Owned(format!("{}; {}", pb.msg, suffix.into()));
        pb
    }
}

impl IntoResponse for BizProblem {
    fn into_response(self) -> Response {
        self.body().into_response()
    }
}

impl IntoResponse for ProblemBody {
    fn into_response(self) -> Response {
        let status = StatusCode::from_u16(self.status).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
        (status, [(CONTENT_TYPE, "application/problem+json")], Json(self)).into_response()
    }
}

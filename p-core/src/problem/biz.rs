use crate::model::result::base::BizResult;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use std::borrow::Cow;

/// The code namespace is as follows:
///
/// - ok: ok
/// - sys.*: system & infrastructure; eg: json, io, network, middleware
/// - common.*: common biz; eg: state, argument, param
/// - auth.*: account & auth; eg: apikey, token
/// - external.*: external service; eg: alipay, wechat
/// - *: Business-line specific; eg: user, product, order
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BizProblem {
    pub code: Cow<'static, str>,
    pub msg: Cow<'static, str>,
}

impl BizProblem {
    pub const OK: Self = Self {
        code: Cow::Borrowed("ok"),
        msg: Cow::Borrowed("ok"),
    };

    pub const UNKNOWN: Self = Self {
        code: Cow::Borrowed("sys.unknown"),
        msg: Cow::Borrowed("unknown"),
    };

    pub const DATA_SERDE: Self = Self {
        code: Cow::Borrowed("sys.serde"),
        msg: Cow::Borrowed("fail to serde"),
    };

    pub const CONSUL: Self = Self {
        code: Cow::Borrowed("sys.consul"),
        msg: Cow::Borrowed("consul operation failed"),
    };

    pub const REDIS: Self = Self {
        code: Cow::Borrowed("sys.redis"),
        msg: Cow::Borrowed("redis operation failed"),
    };

    pub const DB: Self = Self {
        code: Cow::Borrowed("sys.db"),
        msg: Cow::Borrowed("db operation failed"),
    };

    pub const OPENSEARCH: Self = Self {
        code: Cow::Borrowed("sys.opensearch"),
        msg: Cow::Borrowed("opensearch operation failed"),
    };

    pub const NATS: Self = Self {
        code: Cow::Borrowed("sys.nats"),
        msg: Cow::Borrowed("nats operation failed"),
    };

    pub const FILE_IO: Self = Self {
        code: Cow::Borrowed("sys.io.file"),
        msg: Cow::Borrowed("file i/o operation failed"),
    };

    pub const NETWORK: Self = Self {
        code: Cow::Borrowed("sys.network"),
        msg: Cow::Borrowed("network operation failed"),
    };

    pub const RATE_LIMIT: Self = Self {
        code: Cow::Borrowed("sys.rate.limit"),
        msg: Cow::Borrowed("too many request"),
    };

    pub const ILLEGAL_STATE: Self = Self {
        code: Cow::Borrowed("common.state.illegal"),
        msg: Cow::Borrowed("illegal state"),
    };

    pub const ILLEGAL_FORMAT: Self = Self {
        code: Cow::Borrowed("common.format.illegal"),
        msg: Cow::Borrowed("illegal format"),
    };

    pub const ILLEGAL_ARGUMENT: Self = Self {
        code: Cow::Borrowed("common.argument.illegal"),
        msg: Cow::Borrowed("illegal argument"),
    };

    pub const PARAMETER_MISSING: Self = Self {
        code: Cow::Borrowed("common.parameter.missing"),
        msg: Cow::Borrowed("parameter missing"),
    };

    pub const PARAMETER_INVALID: Self = Self {
        code: Cow::Borrowed("common.parameter.invalid"),
        msg: Cow::Borrowed("parameter invalid"),
    };

    pub const PASSWORD_PATTERN: Self = Self {
        code: Cow::Borrowed("auth.password.invalid"),
        msg: Cow::Borrowed("password pattern invalid"),
    };

    pub const PASSWORD_HASH: Self = Self {
        code: Cow::Borrowed("auth.password.hash"),
        msg: Cow::Borrowed("fail to hash password"),
    };

    pub const PASSWORD_VERIFY: Self = Self {
        code: Cow::Borrowed("auth.password.verify"),
        msg: Cow::Borrowed("fail to verify password"),
    };

    pub const APIKEY_MISSING: Self = Self {
        code: Cow::Borrowed("auth.apikey.missing"),
        msg: Cow::Borrowed("apikey missing"),
    };

    pub const APIKEY_INVALID: Self = Self {
        code: Cow::Borrowed("auth.apikey.invalid"),
        msg: Cow::Borrowed("apikey invalid"),
    };

    pub const TOKEN_MISSING: Self = Self {
        code: Cow::Borrowed("auth.token.missing"),
        msg: Cow::Borrowed("token missing"),
    };

    pub const TOKEN_INVALID: Self = Self {
        code: Cow::Borrowed("auth.token.invalid"),
        msg: Cow::Borrowed("token invalid"),
    };

    pub const USER_NOT_EXIST: Self = Self {
        code: Cow::Borrowed("user.not-exist"),
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
    fn into_response(self) -> axum::response::Response {
        BizResult::<()>::from(self).into_response()
    }
}

impl std::fmt::Display for BizProblem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{}] {}", self.code, self.msg)
    }
}

impl std::error::Error for BizProblem {}

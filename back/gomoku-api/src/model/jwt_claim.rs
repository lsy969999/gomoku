use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

// use crate::entity::user::{UserRole, UserStatus};

/**
 * sub (Subject): 토큰의 주체, 일반적으로 사용자 ID
 * iss (Issuer): 토큰을 발행한 주체
 * aud (Audience): 토큰의 수신자, 즉 이 토큰을 사용할 수 있는 대상
 * exp (Expiration Time): 토큰의 만료 시간
 * iat (Issued At): 토큰이 발행된 시간
 * nbf (Not Before): 토큰이 활성화되는 시간
 * jti (JWT ID): 토큰의 고유 식별자
 * scope: 권한 또는 역할 정보
 * email: 사용자 이메일 (선택적)
 * name: 사용자 이름 (선택적)
 *
 * struct Claims {
    aud: String,         // Optional. Audience
    exp: usize,          // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
    iat: usize,          // Optional. Issued at (as UTC timestamp)
    iss: String,         // Optional. Issuer
    nbf: usize,          // Optional. Not Before (as UTC timestamp)
    sub: String,         // Optional. Subject (whom token refers to)
}
 */

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AccessClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub scope: Option<String>,
    pub nick_name: String,
    pub avatar_url: Option<String>,
    pub user_status: String,
    pub user_role: String,
}

impl AccessClaims {
    pub fn new(
        sub: String,
        exp: OffsetDateTime,
        iat: OffsetDateTime,
        scope: Option<String>,
        nick_name: String,
        avatar_url: Option<String>,
        user_status: String,
        user_role: String,
    ) -> Self {
        Self {
            sub,
            exp: exp.unix_timestamp() as usize,
            iat: iat.unix_timestamp() as usize,
            scope,
            nick_name,
            avatar_url,
            user_status,
            user_role,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RefreshClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub scope: Option<String>,
}

impl RefreshClaims {
    pub fn new(
        sub: String,
        exp: OffsetDateTime,
        iat: OffsetDateTime,
        scope: Option<String>,
    ) -> Self {
        Self {
            sub,
            exp: exp.unix_timestamp() as usize,
            iat: iat.unix_timestamp() as usize,
            scope,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsTempClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}
impl WsTempClaims {
    pub fn new(sub: String, exp: OffsetDateTime, iat: OffsetDateTime) -> Self {
        Self {
            sub,
            exp: exp.unix_timestamp() as usize,
            iat: iat.unix_timestamp() as usize,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WsClaims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
    pub connection_id: String,
}
impl WsClaims {
    pub fn new(
        sub: String,
        exp: OffsetDateTime,
        iat: OffsetDateTime,
        connection_id: String,
    ) -> Self {
        Self {
            sub,
            exp: exp.unix_timestamp() as usize,
            iat: iat.unix_timestamp() as usize,
            connection_id,
        }
    }
}

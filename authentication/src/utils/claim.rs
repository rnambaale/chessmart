use std::{sync::LazyLock, time::Duration};

use chrono::Utc;
use jsonwebtoken::{Algorithm, DecodingKey, EncodingKey, Header, TokenData, Validation};

use serde::Deserialize;
use serde::Serialize;
use uuid::Uuid;

pub static DECODE_HEADER: LazyLock<Validation> =
  LazyLock::new(|| Validation::new(Algorithm::RS256));
pub static ENCODE_HEADER: LazyLock<Header> = LazyLock::new(|| Header::new(Algorithm::RS256));

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord, Clone)]
pub struct UserClaims {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // user id
    pub uid: Uuid,
    // session id
    pub sid: Uuid,
}

impl UserClaims {
    pub fn new(duration: Duration, user_id: Uuid, session_id: Uuid) -> Self {
        let now = Utc::now().timestamp();
        Self {
            iat: now,
            exp: now + (duration.as_secs() as i64),
            uid: user_id,
            sid: session_id,
        }
    }

    pub fn decode(
        token: &str,
        key: &DecodingKey,
    ) -> Result<TokenData<Self>, jsonwebtoken::errors::Error> {
        jsonwebtoken::decode::<UserClaims>(token, key, &DECODE_HEADER)
    }

    pub fn encode(&self, key: &EncodingKey) -> Result<String, jsonwebtoken::errors::Error> {
        jsonwebtoken::encode(&ENCODE_HEADER, self, key)
    }
}

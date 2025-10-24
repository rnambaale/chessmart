use std::{sync::LazyLock, time::Duration};

use clap::Parser;
use jsonwebtoken::{DecodingKey, EncodingKey};

use crate::config::TokenSecretConfig;

pub const EXPIRE_SESSION_CODE_SECS: Duration = Duration::from_secs(2000);
pub const EXPIRE_BEARER_TOKEN_SECS: Duration = Duration::from_secs(600);
pub const EXPIRE_REFRESH_TOKEN_SECS: Duration = Duration::from_secs(3600);

pub static ACCESS_TOKEN_ENCODE_KEY: LazyLock<EncodingKey> = LazyLock::new(|| {
  let key = TokenSecretConfig::parse().access_token_private_key;
  EncodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});

pub static REFRESH_TOKEN_ENCODE_KEY: LazyLock<EncodingKey> = LazyLock::new(|| {
  let key = TokenSecretConfig::parse().refresh_token_private_key;
  EncodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});

pub static REFRESH_TOKEN_DECODE_KEY: LazyLock<DecodingKey> = LazyLock::new(|| {
  let key = TokenSecretConfig::parse().refresh_token_public_key;
  DecodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});

pub static ACCESS_TOKEN_DECODE_KEY: LazyLock<DecodingKey> = LazyLock::new(|| {
  let key = TokenSecretConfig::parse().access_token_public_key;
  DecodingKey::from_rsa_pem(key.as_bytes()).unwrap()
});

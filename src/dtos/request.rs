use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct RegisterRequestDto {
    pub email: String,
    pub username: String,
    pub password: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub struct LoginRequestDto {
  pub email: String,
  pub password: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, IntoParams)]
pub struct RefreshTokenRequestDto {
  pub token: String,
}

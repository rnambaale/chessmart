use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Serialize, Deserialize, Default, ToSchema)]
pub struct RegisterResponseDto {
    pub id: String,
    pub email: String,
    pub created_at: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct LoginResponseDto {
  pub jwt: String,
  pub jwt_expires_in: u64,
  pub jwt_refresh: String,
  pub jwt_refresh_expires_in: u64,
}

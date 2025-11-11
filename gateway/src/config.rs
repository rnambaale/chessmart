use std::net::SocketAddr;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(arg_required_else_help(true))]
pub struct Opts {
    #[clap(flatten)]
    pub server: ServerConfig,

    #[clap(flatten)]
    pub tracing: Option<TracingConfig>,

    #[clap(flatten)]
    pub redis: RedisConfig,
}

#[derive(Debug, Clone, Parser)]
pub struct RedisConfig {
    #[clap(long, env = "REDIS_USERNAME")]
    pub username: String,

    #[clap(long, env = "REDIS_PASSWORD")]
    pub password: String,

    #[clap(long, env = "REDIS_PORT")]
    pub port: u16,

    #[clap(long, env = "REDIS_HOST")]
    pub host: String,

    #[clap(long, env = "REDIS_DATABASE_NAME")]
    pub database_name: String,
}

impl Default for RedisConfig {
    fn default() -> Self {
        Self {
            username: "".to_owned(),
            password: "".to_owned(),
            port: 6379,
            host: "127.0.0.1".to_owned(),
            database_name: "".to_owned(),
        }
    }
}

impl RedisConfig {
  pub fn get_url(&self) -> String {
    format!(
      "redis://{username}:{password}@{host}:{port}/{database_name}",
      username = self.username,
      password = self.password,
      host = self.host,
      port = self.port,
      database_name = self.database_name
    )
  }
}

#[derive(Debug, Clone, Parser, Default)]
pub struct TokenSecretConfig {
    #[clap(long, env = "ACCESS_TOKEN_PRIVATE_KEY")]
    pub access_token_private_key: String,

    #[clap(long, env = "ACCESS_TOKEN_PUBLIC_KEY")]
    pub access_token_public_key: String,

    #[clap(long, env = "REFRESH_TOKEN_PRIVATE_KEY")]
    pub refresh_token_private_key: String,

    #[clap(long, env = "REFRESH_TOKEN_PUBLIC_KEY")]
    pub refresh_token_public_key: String,
}

#[derive(Debug, Clone, Parser)]
pub struct ServerConfig {
    #[clap(long, default_value = "[::]:3338", env = "HOST_PORT")]
    pub host_port: SocketAddr,

    #[clap(long, env = "API_PREFIX")]
    pub api_prefix: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host_port: "[::]:3338".to_string().parse().expect("invalid host port"),
            api_prefix: None,
        }
    }
}

#[derive(Debug, Clone, Default, Parser)]
pub struct TracingConfig {
    #[clap(long, env = "API_TRACING_ENDPOINT")]
    pub endpoint: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct ApiConfig {
    pub server: ServerConfig,
    pub tracing: Option<TracingConfig>,
    pub redis: RedisConfig,
    pub token_secret: TokenSecretConfig,
}

impl From<(Opts, TokenSecretConfig)> for ApiConfig {
    fn from((opts, token_secret): (Opts, TokenSecretConfig)) -> Self {
        Self {
            server: opts.server,
            tracing: opts.tracing,
            redis: opts.redis,
            token_secret
        }
    }
}

impl ApiConfig {
    pub fn read_config_with_defaults() -> Self {
        let opts: Opts = Opts::parse();

        let token_secret_config: TokenSecretConfig = TokenSecretConfig::parse();

        (opts, token_secret_config).into()
    }
}

impl ApiConfig {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        server: ServerConfig,
        tracing: Option<TracingConfig>,
        redis: RedisConfig,
        token_secret: TokenSecretConfig,
    ) -> Self {
        Self {
            server,
            tracing,
            redis,
            token_secret,
        }
    }
}

use std::net::SocketAddr;
use clap::Parser;

#[derive(Parser, Debug)]
#[command(arg_required_else_help(true))]
pub struct Opts {
    #[clap(flatten)]
    pub server: ServerConfig,

    #[clap(flatten)]
    pub database: DatabaseConfig,

    #[clap(flatten)]
    pub tracing: Option<TracingConfig>,

    #[clap(flatten)]
    pub redis: RedisConfig,
}

#[derive(Debug, Clone, Parser)]
pub struct DatabaseConfig {
    #[clap(long, env = "DATABASE_URL")]
    pub db_url: String,

    #[clap(long, default_value_t = 5, env = "MAX_DB_CONNECTIONS")]
    pub max_connections: u32,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self {
            db_url: "".to_owned(),
            max_connections: 5,
        }
    }
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

#[derive(Debug, Clone, Parser)]
pub struct ServerConfig {
    #[clap(long, default_value = "[::]:50052", env = "HOST_PORT")]
    pub host_port: SocketAddr,

    #[clap(long, env = "API_PREFIX")]
    pub api_prefix: Option<String>,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host_port: "[::]:50052".to_string().parse().expect("invalid host port"),
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
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
}

impl From<Opts> for ApiConfig {
    fn from(opts: Opts) -> Self {
        Self {
            server: opts.server,
            tracing: opts.tracing,
            database: opts.database,
            redis: opts.redis,
        }
    }
}

impl ApiConfig {
    pub fn read_config_with_defaults() -> Self {
        let opts: Opts = Opts::parse();

        opts.into()
    }
}

impl ApiConfig {
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        server: ServerConfig,
        database: DatabaseConfig,
        tracing: Option<TracingConfig>,
        redis: RedisConfig,
    ) -> Self {
        Self {
            server,
            database,
            tracing,
            redis,
        }
    }
}

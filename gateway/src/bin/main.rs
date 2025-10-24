use std::env;

use bunnychess::{config::{ApiConfig, TracingConfig}, server::state::AppStateBuilder};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::Sampler;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let app_env = match env::var("APP_ENV") {
        Ok(v) if v.trim() == "dev" => AppEnv::Dev,
        _ => AppEnv::Prod,
    };

    println!("Running in {app_env} mode");

    if app_env == AppEnv::Dev {
        match dotenvy::dotenv() {
            Ok(path) => println!(".env read successfully from {}", path.display()),
            Err(e) => panic!("Could not load .env file: {e}"),
        };
    }

    let ApiConfig {
        server,
        database,
        tracing,
        redis,
        token_secret,
    } = ApiConfig::read_config_with_defaults();

    init_tracing(tracing.clone())?;

    let state = AppStateBuilder::new()
        .with_server(Some(server))
        .with_db(Some(database))
        .with_tracing(tracing)
        .with_redis(Some(redis))
        .with_token_secret(Some(token_secret))
        .build()
        .await?;

    bunnychess::server::run_server(state).await
}

fn init_tracing(tr: Option<TracingConfig>) -> anyhow::Result<()> {
    let otlp_tracer = if tr.is_some() {
        let tracer = opentelemetry_otlp::new_pipeline()
            .tracing()
            .with_exporter(
                opentelemetry_otlp::new_exporter().http().with_endpoint(
                    tr.unwrap_or_default()
                        .endpoint
                        .expect("No endpoint for tracing found"),
                ),
            )
            .with_trace_config(
                opentelemetry_sdk::trace::config()
                    .with_sampler(Sampler::AlwaysOn)
                    .with_resource(opentelemetry_sdk::Resource::new(vec![KeyValue::new(
                        "service.name",
                        "bunny-chess",
                    )])),
            )
            .install_batch(opentelemetry_sdk::runtime::Tokio)?;
        Some(tracing_opentelemetry::layer().with_tracer(tracer))
    } else {
        None
    };

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .with(otlp_tracer)
        .try_init()?;
    Ok(())
}

#[derive(Debug, PartialEq, Eq)]
pub enum AppEnv {
    Dev,
    Prod,
}

impl core::fmt::Display for AppEnv {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Dev => write!(f, "dev"),
            Self::Prod => write!(f, "prod"),
        }
    }
}

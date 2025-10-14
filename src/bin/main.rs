use bunnychess::config::{ApiConfig, TracingConfig};
use opentelemetry::KeyValue;
use opentelemetry_otlp::WithExportConfig;
use opentelemetry_sdk::trace::Sampler;
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let ApiConfig {
        server: _,
        database: _,
        tracing,
    } = ApiConfig::read_config_with_defaults();

    init_tracing(tracing.clone())?;

    bunnychess::server::run_server().await
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

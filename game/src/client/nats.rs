use async_nats::jetstream;

use crate::config::NatsConfig;

#[derive(Clone)]
pub struct NatsDB {}

// pub type NatsClient = async_nats::Client;
pub type NatsJetstreamContext = async_nats::jetstream::Context;

impl NatsDB {
    pub async fn new(config: &NatsConfig) -> Result<NatsJetstreamContext, async_nats::Error> {
        let nats_client = async_nats::connect(config.get_url()).await?;

        Ok(jetstream::new(nats_client))
    }
}

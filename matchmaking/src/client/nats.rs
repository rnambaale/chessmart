use async_nats::jetstream;

use crate::config::NatsConfig;

#[derive(Clone)]
pub struct NatsDB {}

// pub type NatsClient = async_nats::Client;
pub type NatsJetstreamContext = async_nats::jetstream::Context;

impl NatsDB {
    pub async fn new(config: &NatsConfig) -> Result<NatsJetstreamContext, async_nats::Error> {

        let NatsConfig { nats_user, nats_password, nats_url } = config;

        let nats_client = async_nats::ConnectOptions::with_user_and_password(
            nats_user.into(),
            nats_password.into(),
        )
            .connect(nats_url)
            .await?;

        Ok(jetstream::new(nats_client))
    }
}

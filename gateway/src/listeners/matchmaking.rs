use std::{str::from_utf8, sync::Arc};

use async_nats::jetstream::{self, consumer::PullConsumer};
use futures::StreamExt;
use shared::events::MatchmakingEvent;
use socketioxide::SocketIo;

use crate::server::state::AppState;

pub async fn matchmaking_consumer(state: Arc<AppState>, socket_io: Arc<SocketIo>) -> Result<(), async_nats::Error> {
    let jetstream = &state.jetstream;

    let stream_name = String::from("matchmaking-publisher");

    let consumer: PullConsumer = jetstream
        .create_stream(jetstream::stream::Config {
            name: stream_name,
            subjects: vec!["chessmart.game.>".into()],
            ..Default::default()
        })
        .await?
        // Then, on that `Stream` use method to create Consumer and bind to it too.
        .create_consumer(jetstream::consumer::pull::Config {
            durable_name: Some("consumer".into()),
            ..Default::default()
        })
        .await?;

    println!("Listening for matchmaking events...");

    // Attach to the messages iterator for the Consumer.
    // The iterator does its best to optimize retrieval of messages from the server.
    let mut messages = consumer.messages().await?.take(10);

    // Iterate over messages.
    while let Some(message) = messages.next().await {
        let message = message?;

        println!(
            "got matchmacking event message on subject {} with payload {:?}",
            message.subject,
            from_utf8(&message.payload)?
        );

        if let Ok(event) = serde_json::from_slice::<MatchmakingEvent>(&message.payload) {
            handle_matchmacking_event(event, socket_io.clone()).await;
        }

        // acknowledge the message
        message.ack().await?;
    }

    Ok(())
}

async fn handle_matchmacking_event(event: MatchmakingEvent, socket_io: Arc<SocketIo>) {
    match event {
        MatchmakingEvent::PendingGameReady(game_ready) => {
            println!("Penidng Game ready: {} vs {} game id: {}", game_ready.account_id_0, game_ready.account_id_1, game_ready.pending_game_id);

            for account_id in [&game_ready.account_id_0, &game_ready.account_id_1] {
                let payload = serde_json::json!({
                    "type": "pending-game-ready",
                    "pendingGameId": game_ready.pending_game_id,
                });

                socket_io
                    .to(account_id.to_owned())
                    .emit("matchmaking:pending-game-ready", &payload)
                    .await.expect("Failed to emit game ready event");
            }
        },
        MatchmakingEvent::PendingGameTimeout(game_timeout) => {
            println!("Game timeout: {} vs {} game id: {}", game_timeout.account_id_0, game_timeout.account_id_1, game_timeout.pending_game_id);

            for account_id in [&game_timeout.account_id_0, &game_timeout.account_id_1] {
                let payload = serde_json::json!({
                    "type": "pending-game-timeout",
                    "pendingGameId": game_timeout.pending_game_id,
                });

                socket_io
                    .to(account_id.to_owned())
                    .emit("matchmaking:pending-game-timeout", &payload)
                    .await.expect("Failed to emit game timeout event");
            }
        },
        MatchmakingEvent::EloChange(elo_change_event) => {
            println!("Elo change : {} account id: {}", elo_change_event.account_id, elo_change_event.elo_change);

            let shared::events::EloChangeEvent {
                account_id,
                ranked,
                new_elo,
                elo_change
            } = elo_change_event;

            if ranked {
                let payload = serde_json::json!({
                    "type": "elo-change",
                    "newElo": new_elo,
                    "eloChange": elo_change
                });

                socket_io
                    .to(account_id)
                    .emit("matchmaking:elo-change", &payload)
                    .await.expect("Failed to emit elo change event")
            }
        }
    }
}

use std::{str::from_utf8, sync::Arc};

use async_nats::jetstream::{self, consumer::PullConsumer};
use futures::StreamExt;
use shared::events::GameEvent;
use socketioxide::SocketIo;

use crate::server::state::AppState;

pub async fn game_consumer(state: Arc<AppState>, socket_io: Arc<SocketIo>) -> Result<(), async_nats::Error> {
    let jetstream = &state.jetstream;

    let stream_name = String::from("game-publisher");

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

    println!("Listening for game events...");

    // Attach to the messages iterator for the Consumer.
    // The iterator does its best to optimize retrieval of messages from the server.
    let mut messages = consumer.messages().await?.take(10);

    // Iterate over messages.
    while let Some(message) = messages.next().await {
        let message = message?;

        println!(
            "got game event message on subject {} with payload {:?}",
            message.subject,
            from_utf8(&message.payload)?
        );

        if let Ok(event) = serde_json::from_slice::<GameEvent>(&message.payload) {
            handle_game_event(event, socket_io.clone()).await;
        }

        // acknowledge the message
        message.ack().await?;
    }

    Ok(())
}

async fn handle_game_event(event: GameEvent, socket_io: Arc<SocketIo>) {
    match event {
        GameEvent::GameStart(game_start) => {
            println!("Game started: {} vs {}", game_start.account_id_0, game_start.account_id_1);

            // Notify both players about the game start
            for account_id in [&game_start.account_id_0, &game_start.account_id_1] {
                let notification = serde_json::json!({
                    "type": "game_start",
                    "gameId": game_start.game_id,
                    "opponent": if account_id == &game_start.account_id_0 {
                        &game_start.account_id_1
                    } else {
                        &game_start.account_id_0
                    }
                });

                socket_io
                    .to(account_id.to_owned())
                    .emit("game:game-start", &notification)
                    .await.expect("Failed to send game event");
            }
        },
        GameEvent::GameStateUpdate(payload) => {
            println!("Game state update: {}", payload.game_id);

            let shared::events::GameStateUpdateEvent {
                game_id,
                account_id,
                r#move,
                seq,
                clocks,
                ..
            } = payload;

            let notification = serde_json::json!({
                "gameId": game_id,
                "accountId": account_id,
                "move": r#move,
                "seq": seq,
                "clocks": clocks,
            });

            socket_io
                .to(game_id.to_owned())
                .emit("game:game-state-update", &notification)
                .await.expect("Failed to send game event");
        }
        GameEvent::GameOver(payload) => {
            println!("Game over: {}", payload.game_id);

            let shared::events::GameOverEvent {
                game_id,
                winner_account_id,
                game_over_reason,
                ..
            } = payload;

            let notification = serde_json::json!({
                "gameId": game_id,
                "winnerAccountId": winner_account_id,
                "gameOverReason": game_over_reason,
            });

            socket_io
                .to(game_id.to_owned())
                .emit("game:game-over", &notification)
                .await.expect("Failed to send game event");
        }
    }
}

# Chessmart

## Scalable, distributed, stateless chess server based on gRPC microservices

#### _Built with [Axum](https://github.com/tokio-rs/axum/) [NATS JetStream](https://docs.nats.io/nats-concepts/jetstream) [PostgreSQL](https://www.postgresql.org/) and [Redis](https://redis.io/)._

## Introduction
This is a toy project designed to experiment with building Rust-based microservices and gRPC, leveraging NATS JetStream and its persistent storage to demonstrate how this stack can build fault tolerant distributed systems. The microservices are stateless and can be scaled horizontally while using Redis to maintain the game state.

This project implements an API server for setting up chess games. Users can:
- create accounts
- matchup with other users for a chess game
- get ranked depending on their win history

Included features:
- 🎮 Multiple game modes and ranked/unranked matches
- ⏳ Matchmaking server
- 💪 Fault-tolerant, persistent queues
- 💬 Chat and live matchmaking queues status

Tech stack:
- 💻 Rust microservices (monorepo)
- 📔 NATS JetStream, PostgreSQL ([sqlx](https://github.com/launchbadge/sqlx)), Redis
- 🗣️ gRPC, WebSocket ([Socketioxide](https://github.com/Totodore/socketioxide)), HTTP
- 🐳 docker compose

### Microservices / Crates
All microservices are built with Rust and are part of a single monorepo. A [shared](./shared) create is used for protobuf definitions and other utilities.

#### Gateway
The [Gateway](./gateway) acts as the public interface of the project. It handles client requests via HTTP/WebSocket and communicates with internal microservices using gRPC.
We use Socketioxide to manage chat messages independently, routing them directly to the intended recipients without involving other services.

#### Authentication
The [Authentication](./authentication) service is responsible for storing user information, processing authentication requests, and issuing signed JWTs and refresh tokens.
It uses Redis to store refresh tokens and perform token rotation, ensuring secure and efficient token management.

#### Matchmaking
The [Matchmaking](./matchmaking) service manages matchmaking queues, pairs players of similar skill levels, and tracks player rankings.
When two players are matched, it contacts the Game service to create a new chess game instance.
The matchmaking algorithm is implemented entirely in Redis using Lua scripts and is triggered at regular intervals. The algorithm uses an Elo search window that dynamically expands over time, prioritizing players who have waited longer in the queue and it can match over 10000 concurrently enqueued players in less than 30 milliseconds.
Lua scripts ensure the atomicity of multiple commands and dynamically computed keys are avoided to maintain scalability in Redis Cluster environments.
The service listens for persistent `chessmart.game.game-over` events from the Game microservice to update player Elo rankings after a match ends.

#### Game
The [Game](./game/) service handles game creation requests and implements the logic for processing game moves and resignation requests.
It uses Lua scripts in combination with a sequence number to perform atomic CAS (Check-And-Set) operations when updating game states.
When a game ends, the service emits a persistent, long-lived game result message (`chessmart.game.game-over`), ensuring that the Matchmaking service processes the result reliably, even in the event of outages. Each `chessmart.game.game-over` event includes metadata containing the players' Elo rankings at the start of the game. This simplifies ranking calculations and eliminates the need to limit NATS queue consumer parallelism to preserve the order of game-over events.

## Quick start
Clone this repository

Start all services by running

```sh
docker compose up -d

cargo run -p authentication
cargo run -p matchmaking
cargo run -p game
cargo run -p gateway
```

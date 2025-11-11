# Bunnychess

#### _Built with [Axum](https://github.com/tokio-rs/axum/) [PostgreSQL](https://www.postgresql.org/) and [Redis](https://redis.io/)._

## Introduction
This is a toy project designed to experiment with building Rust-based micoriservices and gRPC, leveraging NATS JetStream and its persistent storage to demonstrate how this stack can build fault tolerant distributed systems. The microservices are stateless and can be scaled horizontally while using Redis to maintain the game state.

## Quick start
Clone this repository

Start all services by running

```sh
docker compose up -d
```

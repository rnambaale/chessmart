AUTH_DB_URL := "postgres://postgres:postgres@localhost/bunny-chess-auth"

# list all tasks
default:
  @just --list

# install all dependencies
deps:
  cargo install sqlx-cli typos-cli  grcov just

# run the authentication-service
run-authentication *ARGS:
  cd authentication && RUST_BACKTRACE=1 cargo run -- {{ARGS}}

# run the matchmaking-service
run-matchmaking *ARGS:
  cd matchmaking && RUST_BACKTRACE=1 cargo run -- {{ARGS}}

# run the game-service
run-game *ARGS:
  cd game && RUST_BACKTRACE=1 cargo run -- {{ARGS}}

# run the gateway-service
run-authentication *ARGS:
  cd gateway && RUST_LOG=debug RUST_BACKTRACE=1 APP_ENV=dev cargo run -- {{ARGS}}

# runs all tests
run-tests:
  RUST_BACKTRACE=1 cargo test --workspace --exclude integrationtests

# checks if docker and docker compose is installed and running
_check-docker:
  #!/usr/bin/env bash
  if ! command -v docker &> /dev/null; then
    >&2 echo 'Error: Docker is not installed.';
    exit 1;
  fi

  if ! command -v docker compose &> /dev/null; then
   >&2 echo 'Error: Docker Compose is not installed.' >&2;
   exit 1;
  fi

  if ! command docker info &> /dev/null; then
    >&2 echo 'Error: Docker is not running.';
    exit 1;
  fi

# runs sqlx prepare
db-prepare:
  cd authentication && \
  cargo sqlx prepare --database-url {{ AUTH_DB_URL }}

# runs sqlx prepare
db-migrate:
  cd authentication && \
  cargo sqlx migrate run --database-url {{ AUTH_DB_URL }}

# creates the postgres database
db-create:
  cd authentication && \
  cargo sqlx database create --database-url {{ AUTH_DB_URL }}

#!/bin/sh
set -e

echo "Starting application $APP ..."
exec cargo run -p $APP

#!/usr/bin/env bash

set -x
set -eo pipefail

# check dependencies
if ! [ -x "$(command -v psql)" ]; then
  echo 'Error: psql is not installed.' >&2
  exit 1
fi

if ! [ -x "$(command -v sqlx)" ]; then
  echo 'Error: sqlx-cli is not installed.' >&2
  exit 1
fi

DB_USER=${POSTGRES_USER:=postgres}
DB_PASSWORD="${POSTGRES_PASSWORD:=password}"
DB_NAME="{POSTGRES_DB:=newsletter}"
DB_PORT="${POSTGRES_PORT:=5432}"
DB_HOST="${POSTGRES_HOST:=localhost}"


# launch postgres using Docker
if [[ -z "${SKIP_DOCKER}" ]]
then
  docker run \
  -e POSTGRES_USER=${DB_USER} \
  -e POSTGRES_PASSWORD=${DB_PASSWORD} \
  -e POSTGRES_DB=${DB_NAME} \
  -p "${DB_PORT}":5432 \
  -d postgres \
  postgres -N 1000
fi


  # keep ping postgres until it is ready to accept commands
export PGPASSWORD="${DB_PASSWORD}"
until psql -h "$DB_HOST" -U "$DB_USER" -p "$DB_PORT" -c '\q'; do
  >&2 echo "Postgres is unavailable - sleeping"
sleep 1
done

>&2 echo "Postgres is up running on port ${DB_PORT}..."

DATABASE_URL="postgresql://${DB_USER}:${DB_PASSWORD}@${DB_HOST}:${DB_PORT}/${DB_NAME}"
export DATABASE_URL
sqlx database create
sqlx migrate run

>&2 echo "Postgres has been migrated, ready to go!"


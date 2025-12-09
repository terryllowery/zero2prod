#!/usr/bin/env bash
set -x
set -eo pipefail

if ! [ -x "$(command -v sqlx)" ]; then
  echo >&2 "Error: sqlx is not installed."
  echo >&2 "Use:"
  echo >&2 "    cargo install --version='~0.8' sqlx-cli --no-default-features --features rustls,postgres"
  echo >&2 "to install it."
  exit 1
fi

# check if customer params are set otherwise use defaults
DB_PORT=${DB_PORT:-5432}
SUPERUSER=${SUPERUSER:=postgres}
SUPERUSER_PASSWORD=${SUPERUSER_PASSWORD:=postgres}
APP_USER=${APP_USER:=app}
APP_USER_PWD=${APP_USER_PWD:-${APP_USER_PASSWORD:=secret}}
APP_USER_DB=${APP_USER_DB:=newsletter}

CONTAINER_NAME=${CONTAINER_NAME:-postgres}

# Launch postgres container using docker
# shellcheck disable=SC1073
if [[ -z "${SKIP_DOCKER}" ]]
then
  docker run \
    --env POSTGRES_USER="$SUPERUSER" \
    --env POSTGRES_PASSWORD="$SUPERUSER_PASSWORD" \
    --health-cmd="pg_isready -U $SUPERUSER" \
    --health-interval=1s \
    --health-timeout=5s \
    --health-retries=5 \
    --env POSTGRES_DB="$APP_USER_DB" \
    --publish "$DB_PORT":5432 \
    --detach \
    --name "${CONTAINER_NAME}" \
    postgres -N 1000 || exit 1
  until [ "$(docker inspect -f '{{if .State.Health}}{{.State.Health.Status}}{{else}}no_health{{end}}' "${CONTAINER_NAME}")" = "healthy" ]; do
    >&2 echo "Waiting for PostgreSQL to be ready..."
    sleep 1
  done
  >&2 echo "PostgreSQL is ready."
  # Create app user and database
  CREATE_QUERY="CREATE USER ${APP_USER} WITH PASSWORD '${APP_USER_PWD}';"
  docker exec "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${CREATE_QUERY}"

  # Grant privileges to the app user
  GRANT_QUERY="ALTER USER ${APP_USER} CREATEDB;"
  docker exec "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -c "${GRANT_QUERY}"

  # ensure the target database is owned by the app user and grant schema/database privileges
  docker exec "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -d "${APP_USER_DB}" -c "ALTER DATABASE \"${APP_USER_DB}\" OWNER TO \"${APP_USER}\";"
  docker exec "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -d "${APP_USER_DB}" -c "GRANT ALL PRIVILEGES ON DATABASE \"${APP_USER_DB}\" TO \"${APP_USER}\";"
  docker exec "${CONTAINER_NAME}" psql -U "${SUPERUSER}" -d "${APP_USER_DB}" -c "GRANT ALL ON SCHEMA public TO \"${APP_USER}\";"
fi
# Wait for the database to be ready (handles containers without a health check)






DATABASE_URL=postgres://${APP_USER}:${APP_USER_PWD}@localhost:${DB_PORT}/${APP_USER_DB}
export DATABASE_URL
sqlx database create
sqlx migrate run

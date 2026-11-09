#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")/.."
container=$(docker run -d --rm -e POSTGRES_PASSWORD=orm_fixture_only -p 127.0.0.1::5432 postgres:17)
trap 'docker rm -f "$container" >/dev/null 2>&1 || true' EXIT
ready=false
for attempt in {1..30}; do
  if docker exec "$container" pg_isready -U postgres >/dev/null 2>&1; then ready=true; break; fi
  sleep 1
done
if [[ "$ready" != true ]]; then echo 'Fixture did not become ready' >&2; exit 1; fi
binding=$(docker port "$container" 5432/tcp)
port=${binding##*:}
TEST_DATABASE_URL="postgres://postgres:orm_fixture_only@127.0.0.1:$port/postgres" cargo test --test discovery_postgres -- --ignored

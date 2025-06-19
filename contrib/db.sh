if [ "$1" = "--help" ] || ([ "$1" != "--test" ] && [ -n "$1" ]); then
  echo "Usage: $0 [OPTIONS]"
  echo "Run a PostgreSQL database container for lista-de-compras"
  echo ""
  echo "Options:"
  echo "  --test    Starts the database without a volume for persistent data"
  echo "  --help    Display this help message"
  [ "$1" = "--help" ] && exit 0 || exit 1
fi

CMD=$(command -v docker || command -v podman || echo "")
if [ -z "$CMD" ]; then
  echo "Error: Neither docker nor podman found in PATH" >&2
  exit 1
fi

CONTAINER=bot-na-lista-db
USERNAME=bot
PASSWORD=lista
DATABASE=bot_na_lista
PORT=5432

VOLUME="-v ./pgdata:/var/lib/postgresql/data"
if [ "$1" = "--test" ]; then
  VOLUME=""
fi

$CMD run \
  --name ${CONTAINER} \
  --rm \
  -d \
  -e POSTGRES_USER=${USERNAME} \
  -e POSTGRES_PASSWORD=${PASSWORD} \
  -e POSTGRES_DB=${DATABASE} \
  ${VOLUME} \
  -p ${PORT}:5432 \
  postgres:16.1-bookworm

echo "Postgres running at postgres://${USERNAME}:${PASSWORD}@0.0.0.0:${PORT}/${DATABASE}?sslmode=disable"
echo "To stop it, use: ${CMD} stop ${CONTAINER}"

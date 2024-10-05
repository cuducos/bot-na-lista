if [ "$1" = "--help" ] || ([ "$1" != "--test" ] && [ -n "$1" ]); then
  echo "Usage: $0 [OPTIONS]"
  echo "Run a PostgreSQL database container for lista-de-compras"
  echo ""
  echo "Options:"
  echo "  --test    Starts the database without a volume for persistent data"
  echo "  --help    Display this help message"
  [ "$1" = "--help" ] && exit 0 || exit 1
fi

VOLUME="-v ./pgdata:/var/lib/postgresql/data"
if [ "$1" = "--test" ]; then
  VOLUME=""
fi

docker run \
  --name bot-na-lista-db \
  --rm \
  -d \
  -e POSTGRES_USER=bot \
  -e POSTGRES_PASSWORD=lista \
  -e POSTGRES_DB=bot_na_lista \
  ${VOLUME} \
  -p 5432:5432 \
  postgres:16.1-bookworm

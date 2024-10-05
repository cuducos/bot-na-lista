# Bot na Lista

Bot na Lista bot (Portuguese word play for _add to the list_) made with 💜 by [Cuducos](https://github.com/cuducos) and [Flávia](https://github.com/Flaviasv).

Send a message or add [`@bot_na_lista_bot`](https://t.me/bot_na_lista_bot) to a group to get started.

| Comando | Descrição |
|---|---|
| `<string>` | Adiciona `<string>` à lista de compras |
| `<number>` | Remove ítem número `<number>` da lista de compras |
| `/view` | Mostra a lista de compras |

## Environment Variables

| Variable | Description |
|---|---|
| `DATABASE_URL` | Credentials for a PostgreSQL database |
| `TELOXIDE_TOKEN` | Telegram bot token | 
| `PORT` | Bot webhook port (optional) |
| `HOST` | Bot webhook host (optional) |

If the environment variables `PORT` and `HOST` are set, the bot starts as a webhook. Otherwise, it starts as a polling bot.

### Database

If you need a quick database **for development**, `./contrib/db.sh` starts a Docker container with one at `
postgres://bot:lista@0.0.0.0:5432/bot_na_lista?sslmode=disable`.

If you need one **for tests**, `./contrib/db.sh --test` starts a Docker container with no data persistence at the same URI.

To stop the database use `docker stop bot-na-lista-db`.

## Contributing

```console
$ cargo fmt
$ cargo clippy --fix
$ cargo test -- --test-threads 1
```


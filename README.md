# Lista de Compras

Lista de compras de [Cuducos](https://github.com/cuducos) e [Flávia](https://github.com/Flaviasv).

## Comandos do bot

| Comando | Descrição |
|---|---|
| `/add <item>` | Adiciona `<item>` à lista de compras |
| `/remove <item>` | Remove `<item>` da lista de compras, sendo que este pode ser o número ou o nome do item |
| `/view` | Mostra a lista de compras |

## Variáveis de ambiente

| Variável | Descrição |
|---|---|
| `FLADUCOS_ALLOWED_USERS` | Usuários autorizados a utilizar o bot, separados por vírgula |
| `FLADUCOS_BOT_TOKEN` | Token de acesso de um bot do Telegram |
| `FLADUCOS_GOOGLE_TOKEN` | Chaves JSON de acesso a uma [conta de serviço do Google para o `gspread`](https://docs.gspread.org/en/v5.1.1/oauth2.html#authentication) |
| `FLADUCOS_SPREADSHEET_ID` | ID de uma planilha no Google Sheets |
| `FLADUCOS_WORKSHEET_NAME` | Nome de uma aba da planilha |
| `PORT` | Porta do _webhook_ do bot (opcional) |
| `WEBHOOK_URL` | URL do _webhook_ dp bot (opcional) |

## Instruções

```console
$ python -m venv .venv
$ python .venv/bin/pip install -r requirements.txt
$ python .venv/bin/python bot.py
```

# Lista de Compras

Lista de compras de [Cuducos](https://github.com/cuducos) e [Flávia](https://github.com/Flaviasv).

## Variáveis de ambiente

| Variável | Descrição |
|---|---|
| `FLADUCOS_BOT_TOKEN` | Token de acesso de um bot do Telegram |
| `FLADUCOS_GOOGLE_TOKEN` | Chaves JSON de acesso a uma [conta de serviço do Google para o `gspread`](https://docs.gspread.org/en/v5.1.1/oauth2.html#authentication) |

## Configurações no código

| Variável | Arquivo | Descrição |
|--|---|---|
| `ALLOWED_USERS` | `bot.py` | Usuários autorizados a utilizar o bot |
| `SPREADSHEET_ID` | `sheet.py` | ID de uma planilha no Google Sheets |
| `WORKSHEET_NAME` | `sheet.py` | Nome de uma aba da planilha |

## Instruções

```console
$ python -m venv .venv
$ python .venv/bin/pip install -r requirements.txt
$ python .venv/bin/python bot.py
```

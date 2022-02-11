import os

ALLOWED_USERS = set(u.strip() for u in os.environ["FLADUCOS_ALLOWED_USERS"].split(","))
BOT_TOKEN = os.environ["FLADUCOS_BOT_TOKEN"]
GOOGLE_TOKEN = os.environ["FLADUCOS_GOOGLE_TOKEN"]
SPREADSHEET_ID = os.environ["FLADUCOS_SPREADSHEET_ID"]
WORKSHEET_NAME = os.environ["FLADUCOS_WORKSHEET_NAME"]
WEBHOOK = {
    "listen": "0.0.0.0",
    "port": os.environ.get("PORT"),
    "url_path": BOT_TOKEN,
    "webhook_url": os.environ.get("WEBHOOK_URL", "") + BOT_TOKEN,
}

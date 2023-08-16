"""Me envie uma mensagem.

* Se a mensagem for um número, eu vou excluir o ítem da lista que tem esse número.
* Se a mensagem não for um número, eu vou adicionar o ítem na lista.

Use /view para ver a lista."""

from sys import argv, exit

from telegram.ext import ApplicationBuilder, CommandHandler, MessageHandler, filters

import settings
import sheet


MODES = {"poll", "web"}


def authorized(func):
    def wrapper(update, *args, **kwargs):
        if update.message.chat.username not in settings.ALLOWED_USERS:
            return

        return func(update)

    return wrapper


@authorized
async def help(update):
    await update.message.reply_text(__doc__)


@authorized
async def add_or_remove_item(update):
    item = update.message.text.strip()
    spreadsheet = sheet.spreadsheet()

    try:
        item = int(item)
        action = sheet.remove
    except ValueError:
        action = sheet.add

    action(spreadsheet, item)
    reply = sheet.view(spreadsheet)
    await update.message.reply_text(reply)


@authorized
async def view(update):
    spreadsheet = sheet.spreadsheet()
    reply = sheet.view(spreadsheet)
    await update.message.reply_text(reply)


def get_mode():
    def available():
        modes = ", ".join(sorted(MODES))
        print(f"Available modes are: {modes}.")

    try:
        mode = argv[1].lower()
    except IndexError:
        print(f"Missing `mode` argument. Try python {__file__} <MODE>")
        available()
        exit(1)

    if mode not in MODES:
        print(f"Unknown mode {mode}.")
        available()
        exit(1)

    return mode


def main():
    mode = get_mode()
    app = ApplicationBuilder().token(settings.BOT_TOKEN).build()
    app.add_handler(CommandHandler("view", view))
    app.add_handler(CommandHandler("help", help))
    app.add_handler(MessageHandler(filters.ALL, add_or_remove_item))

    if mode == "poll":
        app.run_polling()
    if mode == "web":
        app.run_webhook(**settings.WEBHOOK)
    app.idle()


if __name__ == "__main__":
    main()

from sys import argv, exit

from telegram.ext import Updater, CommandHandler

import settings
import sheet


MODES = {"poll", "web"}


def to_item(update):
    try:
        pos = update.message.text.index(" ")
    except ValueError:
        return None
    return update.message.text[pos + 1 :].strip()


def authorized(update):
    return update.message.chat.username in settings.ALLOWED_USERS


def add(update, _):
    if not authorized(update):
        return

    item = to_item(update)
    if not item:
        return

    spreadsheet = sheet.spreadsheet()
    sheet.add(spreadsheet, item)
    reply = sheet.view(spreadsheet)
    update.message.reply_text(reply)


def remove(update, _):
    if not authorized(update):
        return

    item = to_item(update)
    if not item:
        return

    spreadsheet = sheet.spreadsheet()
    sheet.remove(spreadsheet, item)
    reply = sheet.view(spreadsheet)
    update.message.reply_text(reply)


def view(update, _):
    if not authorized(update):
        return

    spreadsheet = sheet.spreadsheet()
    reply = sheet.view(spreadsheet)
    update.message.reply_text(reply)


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
    updater = Updater(settings.BOT_TOKEN)
    updater.dispatcher.add_handler(CommandHandler("add", add))
    updater.dispatcher.add_handler(CommandHandler("remove", remove))
    updater.dispatcher.add_handler(CommandHandler("view", view))
    if mode == "poll":
        updater.start_polling()
    if mode == "web":
        updater.start_webhook(**settings.WEBHOOK)
    updater.idle()


if __name__ == "__main__":
    main()

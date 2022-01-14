import os

from telegram.ext import Updater, CommandHandler

import sheet


ALLOWED_USERS = {"cuducos", "Flavia_sv", "gabrilas"}


def to_item(update):
    pos = update.message.text.index(" ")
    return update.message.text[pos + 1 :]


def authorized(update):
    try:
        return update.message.chat.username in ALLOWED_USERS
    except:
        import ipdb

        ipdb.set_trace()


def add(update, _):
    if not authorized(update):
        return

    item = to_item(update)
    spreadsheet = sheet.spreadsheet()
    sheet.add(spreadsheet, item)
    reply = sheet.view(spreadsheet)
    update.message.reply_text(reply)


def remove(update, _):
    if not authorized(update):
        return

    item = to_item(update)
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


updater = Updater(os.environ["FLADUCOS_BOT_TOKEN"])
updater.dispatcher.add_handler(CommandHandler("add", add))
updater.dispatcher.add_handler(CommandHandler("remove", remove))
updater.dispatcher.add_handler(CommandHandler("view", view))
updater.start_polling()
updater.idle()

import os

from telegram.ext import Updater, CommandHandler

import settings
import sheet


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


updater = Updater(settings.BOT_TOKEN)
updater.dispatcher.add_handler(CommandHandler("add", add))
updater.dispatcher.add_handler(CommandHandler("remove", remove))
updater.dispatcher.add_handler(CommandHandler("view", view))
updater.start_polling()
updater.idle()

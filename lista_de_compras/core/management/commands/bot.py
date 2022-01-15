from django.conf import settings
from django.core.management.base import BaseCommand
from telegram.ext import CommandHandler, Updater

from lista_de_compras.core.handler import ACTIONS, HandlerFactory


class Command(BaseCommand):
    help = "Runs the bot continuously"

    def handle(self, *args, **options):
        updater = Updater(settings.FLADUCOS_BOT_TOKEN)
        for action in ACTIONS:
            handler = CommandHandler(action, HandlerFactory.as_func)
            updater.dispatcher.add_handler(handler)
        updater.start_polling()
        updater.idle()

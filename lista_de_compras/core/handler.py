from lista_de_compras.core.manager import (
    CapacityError,
    Manager,
    PersonInAnotherShoppingListError,
)
from lista_de_compras.core.models import ShoppingList

ACTIONS = {"add", "remove", "view", "who"}


class UnknownActionError(Exception):
    def __init__(self, action):
        options = ", ".join(sorted(ACTIONS))
        msg = f"Unknown action {action}, options are: {options}"
        return super().__init__(msg)


class HandlerFactory:
    def __init__(self, manager, update):
        self.manager = manager
        self.value = update.message.text.strip()
        self.action, self.item, self.person = self.parse_action_and_argument()
        if self.action not in ACTIONS:
            raise UnknownActionError(self.action)

    def parse_action_and_argument(self):
        if not self.value:
            return None, None, None

        parts = self.value.split(" ", 1)
        action = parts[0][1:]
        if len(parts) == 1:
            return action, None, None

        argument = parts[1]
        if argument.startswith("@"):
            return action, None, argument

        return action, argument, None

    def __call__(self):
        if self.action == "add":
            if self.person:
                self.manager.add_person(self.person)
            if self.item:
                self.manager.add_item(self.item)

        if self.action == "remove":
            if self.person:
                self.manager.remove_person(self.person)
            if self.item:
                self.manager.remove_item(self.item)

        if self.action == "view" or self.item:
            return self.manager.view()

        if self.action == "who" or self.person:
            return self.manager.who()

    @classmethod
    def as_func(cls, update, _):
        manager = Manager(ShoppingList, update)

        try:
            adapter = cls(manager, update)
        except UnknownActionError:
            options = (f"/{action}" for action in sorted(ACTIONS))
            update.message.reply_text(f"Please use the commands: {options}.")
            return

        try:
            reply = adapter()
        except PersonInAnotherShoppingListError:
            msg = (
                f"@{adapter.person} is in another shopping list, and a person "
                "cannot be in more than one shoppig list simultaneously. Ask "
                "them to leave the other shoping list with "
                f"`/remove @{adapter.person}` and try to add them again."
            )
            update.message.reply_text(msg)
            return

        except CapacityError:
            msg = (
                "This shopping list has reached the maximum capacioty of "
                f"@{ShoppingList.MAX_PEOPLE} people. Cannot add "
                f"@{adapter.person} until someone is removed."
            )
            update.message.reply_text(msg)
            return

        update.message.reply_text("\n".join(reply))

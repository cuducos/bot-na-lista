from django.test import TestCase

from lista_de_compras.core.manager import Manager
from lista_de_compras.core.models import ShoppingList


class TestManager(TestCase):
    def setUp(self):
        self.person, self.another_person = "dummy", "too_dummy"
        self.item, self.another_item = "Banana", "Kiwi"
        ShoppingList.objects.create(
            people={self.person: None},
            items={self.item: None},
        )
        self.manager = Manager(ShoppingList, self.person)

    def _count(self, key):
        return len(getattr(self.manager.shopping_list, key))

    def count_items(self):
        return self._count("items")

    def count_people(self):
        return self._count("people")

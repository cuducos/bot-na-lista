from lista_de_compras.core.manager import Manager
from lista_de_compras.core.models import ShoppingList
from lista_de_compras.core.tests.test_manager import TestManager


class TestManagerInit(TestManager):
    def test_invader_cannot_create_manager(self):
        with self.assertRaises(ShoppingList.DoesNotExist):
            Manager(ShoppingList, self.another_person)

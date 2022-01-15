from lista_de_compras.core.models import ShoppingList
from lista_de_compras.core.tests.test_manager import TestManager


class TestManagerRemovePerson(TestManager):
    def test_remove_person_excludes_shopping_list_if_last_person(self):
        pk = self.manager.shopping_list.id
        self.manager.remove_person(self.person)
        self.assertFalse(ShoppingList.objects.filter(pk=pk).exists())

    def test_remove_person_decrease_count_when_person_is_removed(self):
        self.manager.add_person(self.another_person)
        initial_count = self.count_people()
        self.manager.remove_person(self.another_person)
        self.assertEqual(self.count_people(), initial_count - 1)
        self.assertNotIn(self.another_person, self.manager.view())

    def test_remove_person_does_not_decrease_count_when_person_is_not_found(self):
        initial_count = self.count_people()
        self.manager.remove_person(self.another_person)
        self.assertEqual(self.count_people(), initial_count)
        self.assertNotIn(self.another_person, self.manager.view())

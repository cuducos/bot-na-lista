from random import choice
from string import ascii_letters

from lista_de_compras.core.manager import (
    CapacityError,
    PersonInAnotherShoppingListError,
)
from lista_de_compras.core.models import ShoppingList
from lista_de_compras.core.tests.test_manager import TestManager


class TestManagerAddPerson(TestManager):
    def test_add_person_increases_count_when_person_is_added(self):
        initial_count = self.count_people()
        self.manager.add_person(self.another_person)
        self.assertEqual(self.count_people(), initial_count + 1)
        self.assertIn(self.another_person, self.manager.who())

    def test_add_person_does_not_increase_count_when_person_is_already_present(self):
        initial_count = self.count_people()
        self.manager.add_person(self.person)
        self.assertEqual(self.count_people(), initial_count)
        self.assertIn(self.person, self.manager.who())

    def test_add_person_returns_error_when_person_is_already_in_another(self):
        initial_count = self.count_people()
        ShoppingList.objects.create(people={self.another_person: None})
        with self.assertRaises(PersonInAnotherShoppingListError):
            self.assertFalse(self.manager.add_person(self.another_person))
        self.assertEqual(self.count_people(), initial_count)

    def test_add_person_returns_error_when_full_of_people(self):
        while self.count_people() < ShoppingList.MAX_PEOPLE:
            person = "".join(choice(ascii_letters) for _ in range(64))
            self.manager.add_person(person)

        with self.assertRaises(CapacityError):
            self.manager.add_person(self.another_person)

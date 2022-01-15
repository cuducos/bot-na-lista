from lista_de_compras.core.tests.test_manager import TestManager


class TestManagerWho(TestManager):
    def test_who_returns_one_person(self):
        self.assertEqual(self.manager.who(), (self.person,))

    def test_who_returns_two_people(self):
        self.manager.add_person(self.another_person)
        self.assertEqual(self.manager.who(), (self.person, self.another_person))

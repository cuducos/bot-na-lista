from lista_de_compras.core.tests.test_manager import TestManager


class TestManagerAddItem(TestManager):
    def test_add_item_increases_count_when_item_is_added(self):
        initial_count = self.count_items()
        self.manager.add_item(self.another_item)
        self.assertEqual(self.count_items(), initial_count + 1)
        self.assertIn(self.another_item, self.manager.view())

    def test_add_item_does_not_increase_count_when_item_is_already_present(self):
        initial_count = self.count_items()
        self.manager.add_item(self.item)
        self.assertEqual(self.count_items(), initial_count)
        self.assertIn(self.item, self.manager.view())

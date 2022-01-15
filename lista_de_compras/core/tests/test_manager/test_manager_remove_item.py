from lista_de_compras.core.tests.test_manager import TestManager


class TestManagerRemoveItem(TestManager):
    def test_remove_item_decreases_count_when_item_is_removed(self):
        initial_count = self.count_items()
        self.manager.remove_item(self.item)
        self.assertEqual(self.count_items(), initial_count - 1)
        self.assertNotIn(self.item, self.manager.view())

    def test_remove_item_does_not_decrease_count_when_item_is_not_found(self):
        initial_count = self.count_items()
        self.manager.remove_item(self.another_item)
        self.assertEqual(self.count_items(), initial_count)
        self.assertNotIn(self.another_item, self.manager.view())

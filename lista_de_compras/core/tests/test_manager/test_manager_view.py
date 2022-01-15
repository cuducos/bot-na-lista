from lista_de_compras.core.tests.test_manager import TestManager


class TestManagerView(TestManager):
    def test_view_returns_one_item(self):
        self.assertEqual(self.manager.view(), (self.item,))

    def test_view_returns_two_items(self):
        self.manager.add_item(self.another_item)
        self.assertEqual(self.manager.view(), (self.item, self.another_item))

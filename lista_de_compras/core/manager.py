class PersonInAnotherShoppingListError(Exception):
    def __init__(self, person):
        msg = f"@{person} is already in another shopping list"
        super().__init__(msg)


class CapacityError(Exception):
    def __init__(self, count):
        msg = f"Shopping list is at maximum capacity with {count} people"
        super().__init__(msg)


class Manager:
    def __init__(self, data_storage, person):
        self.data_storage = data_storage
        self.shopping_list = data_storage.for_person(person)

    def add_item(self, item):
        self.shopping_list.items[item] = None
        return self.shopping_list.save()

    def remove_item(self, item):
        if item not in self.shopping_list.items.keys():
            return False

        del self.shopping_list.items[item]
        return self.shopping_list.save()

    def view(self):
        return tuple(self.shopping_list.items.keys())

    def is_person_in_another_shopping_list(self, person):
        try:
            self.data_storage.for_person(person)
        except self.data_storage.DoesNotExist:
            return False
        return True

    def add_person(self, person):
        if person in self.shopping_list.people.keys():
            return False

        if self.is_person_in_another_shopping_list(person):
            raise PersonInAnotherShoppingListError(person)

        if len(self.shopping_list.people) >= self.shopping_list.MAX_PEOPLE:
            raise CapacityError(self.shopping_list.MAX_PEOPLE)

        self.shopping_list.people[person] = None
        return self.shopping_list.save()

    def remove_person(self, person):
        if person not in self.shopping_list.people.keys():
            return False

        del self.shopping_list.people[person]
        if not self.shopping_list.people:
            return self.shopping_list.delete()

        return self.shopping_list.save()

    def who(self):
        return tuple(self.shopping_list.people.keys())

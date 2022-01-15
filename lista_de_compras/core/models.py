from django.contrib.postgres.fields import HStoreField
from django.db import models


class ShoppingList(models.Model):
    MAX_PEOPLE = 16

    people = HStoreField(db_index=True)
    items = HStoreField(default=dict)

    @classmethod
    def for_person(cls, person):
        data = {"people": {person: None}}
        return cls.objects.get_or_create(defaults=data, people__has_key=person)

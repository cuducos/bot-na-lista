# Lista de Compras

Telegram bot to manage a shopping list, collectively! _Lista de Compras_ is the Portuguese expression for _shopping list_.

## Bots commands

### `/view`

Shows the shopping list.

### `/who`

Shows the people with access to the shopping list.

### `/add <value>`

#### If `<value>` starts with `@`

E.g.: `/add @cuducos`.

Gives that Telegram user access to the shopping list. Each shopping list can have **up to 16 people**, and each person can be **only at one shopping list** at a time.

**Important** to highlight that everyone is allowed to add more people.

#### If `<value>` does not start with `@`

E.g.: `/add Banana`.

Adds `<value>` to the shopping list.

### `/remove <value>`

#### If `<value>` starts with `@`

E.g.: `/remove @cuducos`.

Removes access to the shopping list from that Telegram user.

**Important** to highlight that:
* everyone is allowed to remove others and to remove themselves;
* and when the last one removes themselves from the shopping list, i.e. the list has no people left, the **list is automatically deleted**.

#### If `<value>` does not start with `@`

Removes `<value>` from the shopping list.

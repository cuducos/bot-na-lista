import os
import json
from base64 import b64decode

from gspread import service_account_from_dict


SPREADSHEET_ID = "1UCoGKlHgA3ZaHfnHzhSEibGOoYExVCVKkdDsdnyjbts"
WORKSHEET_NAME = "Lista de Compras"


def credentials():
    encoded = os.environ["FLADUCOS_GOOGLE_TOKEN"]
    decoded = b64decode(encoded)
    return json.loads(decoded)


def spreadsheet():
    sheets = service_account_from_dict(credentials())
    spreadsheet = sheets.open_by_key(SPREADSHEET_ID)
    return spreadsheet.worksheet(WORKSHEET_NAME)


def add(sheet, item):
    if sheet.find(item):
        return None
    return sheet.append_row([item])


def remove(sheet, item):
    for cell in sheet.findall(item):
        sheet.delete_rows(cell.row, cell.row)


def view(sheet):
    items = sheet.col_values(1)
    return "\n".join(items)

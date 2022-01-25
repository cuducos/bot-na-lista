import json
from base64 import b64decode

from gspread import service_account_from_dict

import settings


def credentials():
    encoded = settings.GOOGLE_TOKEN
    decoded = b64decode(encoded)
    return json.loads(decoded)


def spreadsheet():
    sheets = service_account_from_dict(credentials())
    spreadsheet = sheets.open_by_key(settings.SPREADSHEET_ID)
    return spreadsheet.worksheet(settings.WORKSHEET_NAME)


def add(sheet, item):
    cleaned = item.strip()
    if sheet.find(cleaned):
        return None
    return sheet.append_row([cleaned])


def remove(sheet, item):
    for cell in sheet.findall(item.strip()):
        sheet.delete_rows(cell.row, cell.row)


def view(sheet):
    items = sheet.col_values(1)
    return "\n".join(items)

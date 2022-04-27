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

    remove(sheet, "")  # clean-up empty line (if exists)
    return sheet.append_row([cleaned])


def remove(sheet, item):
    if isinstance(item, int):
        sheet.update_cell(item, 1, "")
        sheet.delete_rows(item, item)
        return

    for cell in sheet.findall(item.strip()):
        sheet.update_cell(cell.row, cell.col, "")
        sheet.delete_rows(cell.row, cell.row)


def view(sheet):
    items = sheet.col_values(1)
    cleaned = (f"{count}. {item.strip()}" for count, item in enumerate(items, 1))
    return "\n".join(item for item in cleaned if item)

ALTER TABLE list ADD COLUMN is_empty BOOLEAN NOT NULL DEFAULT TRUE;
UPDATE list SET is_empty = (items = '{}');
CREATE INDEX idx_list_is_empty ON list(is_empty);

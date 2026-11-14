-- Initial SQLite database schema for template-dtnpm-mod
CREATE TABLE IF NOT EXISTS module_items (
    id TEXT PRIMARY KEY,
    source TEXT NOT NULL,
    destination TEXT NOT NULL,
    title TEXT NOT NULL,
    content TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    is_valid_bpsec INTEGER NOT NULL DEFAULT 1
);

CREATE INDEX IF NOT EXISTS idx_module_items_created_at ON module_items(created_at);

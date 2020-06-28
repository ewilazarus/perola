CREATE TABLE perl (
    id         INTEGER PRIMARY KEY AUTOINCREMENT,
    author     VARCHAR(255),
    content    TEXT NOT NULL,
    context    TEXT,
    created_at DATETIME NOT NULL
);

CREATE INDEX idx_author ON perl(author);
CREATE INDEX idx_created_at ON perl(created_at);
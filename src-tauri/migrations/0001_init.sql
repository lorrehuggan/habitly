CREATE TABLE IF NOT EXISTS habits (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    description TEXT,
    created DATE NOT NULL,
    color TEXT,
    streak INTEGER,
    category TEXT NOT NULL,
    status TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS commits (
    id TEXT PRIMARY KEY,
    message TEXT,
    image TEXT,
    created TEXT,
    status TEXT,
    completions INTEGER,
    habit_id TEXT,
    FOREIGN KEY (habit_id) REFERENCES habits (id)
);

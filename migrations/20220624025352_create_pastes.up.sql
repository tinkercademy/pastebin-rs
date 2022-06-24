-- Add up migration script here
CREATE TABLE pastes (
                        id INTEGER PRIMARY KEY AUTOINCREMENT NOT NULL,
                        created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
                        content TEXT NOT NULL
)
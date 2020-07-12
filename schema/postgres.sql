CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    preferences JSONB NOT NULL,
    created_at TIMESTAMP(3) NOT NULL,
);

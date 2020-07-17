CREATE TABLE IF NOT EXISTS customer (
    id UUID PRIMARY KEY,
    username TEXT UNIQUE NOT NULL,
    preferences JSONB NOT NULL,
    created_at TIMESTAMP(3) NOT NULL
);

CREATE TABLE IF NOT EXISTS email (
    id UUID PRIMARY KEY,
    customer_id UUID NOT NULL REFERENCES customer (id),
    email TEXT UNIQUE NOT NULL,
    created_at TIMESTAMP(3) NOT NULL
);

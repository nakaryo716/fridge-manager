CREATE TABLE IF NOT EXISTS users (
    user_id        SERIAL PRIMARY KEY,
    user_name      TEXT NOT NULL,
    mail           TEXT NOT NULL,
    password       TEXT NOT NULL
);

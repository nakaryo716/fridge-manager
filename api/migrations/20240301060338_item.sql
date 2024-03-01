CREATE TABLE IF NOT EXISTS fridge-db
(
    id          SERIAL PRIMARY KEY,
    name        TEXT    NOT NULL,
    deadline
    used        BOOLEAN NOT NULL DEFAULT false
)

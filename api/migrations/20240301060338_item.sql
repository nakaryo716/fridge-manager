CREATE TABLE IF NOT EXISTS item (
    id          SERIAL  PRIMARY KEY,
    name        TEXT    NOT NULL,
    year        INTEGER NOT NULL,
    month       INTEGER NOT NULL,
    day         INTEGER NOT NULL,
    used        BOOLEAN NOT NULL DEFAULT FALSE
);

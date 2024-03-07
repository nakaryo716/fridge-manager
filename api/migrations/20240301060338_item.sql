CREATE TABLE IF NOT EXISTS item (
    id                  SERIAL  PRIMARY KEY,
    name                TEXT    NOT NULL,
    expiration_date     DATE    NOT NULL,
    used                BOOLEAN NOT NULL DEFAULT FALSE
);

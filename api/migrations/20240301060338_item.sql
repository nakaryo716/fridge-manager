CREATE TYPE EXDATE AS (
    year    INT,
    month   INT,
    day     INT
);

CREATE TABLE IF NOT EXISTS item (
    id              SERIAL PRIMARY KEY,
    name            TEXT    NOT NULL,
    expiration_date EXDATE NOT NULL,
    used            BOOLEAN NOT NULL DEFAULT false
);

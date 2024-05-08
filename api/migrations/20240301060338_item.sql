CREATE TABLE IF NOT EXISTS item (
    food_id        SERIAL  PRIMARY KEY,
    food_name      TEXT    NOT NULL,
    expiration     DATE    NOT NULL,
    used           BOOLEAN NOT NULL DEFAULT FALSE
);

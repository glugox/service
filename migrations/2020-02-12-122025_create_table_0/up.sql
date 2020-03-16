CREATE TABLE services
(
    id       SERIAL PRIMARY KEY,
    name     TEXT NOT NULL UNIQUE,
    url      TEXT NOT NULL UNIQUE,
    active   BOOLEAN
);

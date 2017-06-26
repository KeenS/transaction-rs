-- -*- mode: sql; sql-product: postgres; -*-
-- Your SQL goes here

CREATE TABLE users
(
        id bigserial PRIMARY KEY NOT NULL,
        name varchar NOT NULL
);

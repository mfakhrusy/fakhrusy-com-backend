-- Your SQL goes here

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    email text NOT NULL,
    hashed_password text NOT NULL,
    salt text NOT NULL,
    full_name text
)
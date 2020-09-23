-- Your SQL goes here
CREATE TABLE users (
    username VARCHAR PRIMARY KEY NOT NULL,
    display_name VARCHAR NOT NULL,
    hashed_password VARCHAR NOT NULL,
    role VARCHAR NOT NULL
);

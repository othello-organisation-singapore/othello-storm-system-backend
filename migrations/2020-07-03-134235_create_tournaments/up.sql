-- Your SQL goes here

CREATE TABLE tournaments (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    tournament_type VARCHAR NOT NULL,
    country VARCHAR NOT NULL,
    creator VARCHAR NOT NULL REFERENCES users(username),
    joueurs json NOT NULL,
    meta_data json NOT NULL
);

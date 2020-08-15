-- Your SQL goes here

CREATE TABLE joueurs (
    id SERIAL PRIMARY KEY,
    timestamp timestamp NOT NULL,
    content json NOT NULL
);

CREATE TABLE tournaments (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    country VARCHAR NOT NULL,
    creator VARCHAR NOT NULL REFERENCES users(username),
    joueurs_id INT NOT NULL REFERENCES joueurs(id),
    meta_data json NOT NULL
);

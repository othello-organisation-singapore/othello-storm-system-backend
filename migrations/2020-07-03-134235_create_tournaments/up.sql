-- Your SQL goes here
CREATE TABLE tournaments (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    country VARCHAR NOT NULL,
    creator VARCHAR NOT NULL,
    joueurs json NOT NULL,
    meta_data json NOT NULL,
    FOREIGN KEY (creator) REFERENCES users (username)
);

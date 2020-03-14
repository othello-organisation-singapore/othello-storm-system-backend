-- Your SQL goes here
CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL,
    display_name VARCHAR NOT NULL,
    hashed_password VARCHAR NOT NULL,
    role VARCHAR NOT NULL
);

INSERT INTO users VALUES (1, 'chrismaxheart','Samuel Henry Kurniawan',
                          '$2b$08$FX2ZL5AeBMopCYwRa7o9F.jL0D3RvFZOSdnPn33KolL6IkNbgDNW6', 'superuser');

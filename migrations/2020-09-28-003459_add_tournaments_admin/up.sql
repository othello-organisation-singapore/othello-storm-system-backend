-- Your SQL goes here
CREATE TABLE tournaments_admin
(
    id             SERIAL PRIMARY KEY,
    tournament_id  INT     NOT NULL REFERENCES tournaments (id) ON DELETE NO ACTION,
    admin_username VARCHAR NOT NULL REFERENCES users (username) ON DELETE NO ACTION
);

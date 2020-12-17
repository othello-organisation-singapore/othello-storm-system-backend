-- Your SQL goes here

CREATE TABLE players
(
    id            SERIAL PRIMARY KEY,
    tournament_id INT     NOT NULL REFERENCES tournaments (id) ON DELETE NO ACTION,
    joueurs_id    INT     NOT NULL,
    first_name    VARCHAR NOT NULL,
    last_name     VARCHAR NOT NULL,
    country       VARCHAR NOT NULL,
    rating        VARCHAR NOT NULL,
    meta_data     json    NOT NULL
);

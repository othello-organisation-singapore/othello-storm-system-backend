-- Your SQL goes here

CREATE TABLE players
(
    id            SERIAL PRIMARY KEY,
    tournament_id INT     NOT NULL REFERENCES tournaments (id) ON DELETE NO ACTION,
    joueurs_id    VARCHAR NOT NULL,
    first_name    VARCHAR NOT NULL,
    last_name     VARCHAR NOT NULL,
    country       VARCHAR NOT NULL,
    rating        INT     NOT NULL,
    meta_data     json    NOT NULL,
    UNIQUE (tournament_id, joueurs_id)
);

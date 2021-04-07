-- Your SQL goes here

CREATE TABLE rounds
(
    id            SERIAL PRIMARY KEY,
    tournament_id INT     NOT NULL REFERENCES tournaments (id) ON DELETE NO ACTION,
    name          VARCHAR NOT NULL,
    round_type    INT     NOT NULL,
    meta_data     json    NOT NULL
);

CREATE TABLE matches
(
    id              SERIAL PRIMARY KEY,
    round_id        INT  NOT NULL REFERENCES rounds (id) ON DELETE NO ACTION,
    black_player_id INT  NOT NULL REFERENCES players (id) ON DELETE NO ACTION,
    white_player_id INT  NOT NULL REFERENCES players (id) ON DELETE NO ACTION,
    black_score     INT  NOT NULL,
    white_score     INT  NOT NULL,
    meta_data       json NOT NULL
);

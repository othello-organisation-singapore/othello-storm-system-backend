-- Your SQL goes here

ALTER TABLE matches
    DROP CONSTRAINT matches_white_player_id_fkey;
ALTER TABLE matches
    DROP CONSTRAINT matches_black_player_id_fkey;

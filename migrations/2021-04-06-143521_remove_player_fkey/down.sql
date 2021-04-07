-- This file should undo anything in `up.sql`

ALTER TABLE matches
    ADD CONSTRAINT matches_white_player_id_fkey FOREIGN KEY (white_player_id) REFERENCES players (id);
ALTER TABLE matches
    ADD CONSTRAINT matches_black_player_id_fkey FOREIGN KEY (black_player_id) REFERENCES players (id);

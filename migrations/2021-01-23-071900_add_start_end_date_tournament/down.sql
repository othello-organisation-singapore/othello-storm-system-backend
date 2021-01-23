-- This file should undo anything in `up.sql`

ALTER TABLE tournaments DROP COLUMN IF EXISTS start_date;
ALTER TABLE tournaments DROP COLUMN IF EXISTS end_date;

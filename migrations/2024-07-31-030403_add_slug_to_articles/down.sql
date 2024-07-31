-- This file should undo anything in `up.sql`
ALTER TABLE articles
DROP COLUMN slug;

DROP TRIGGER IF EXISTS update_slug_trigger ON articles;

DROP FUNCTION IF EXISTS update_slug();

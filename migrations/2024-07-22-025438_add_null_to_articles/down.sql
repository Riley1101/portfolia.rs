-- This file should undo anything in `up.sql`
ALTER TABLE articles 
ALTER COLUMN updatedAt SET NOT NULL, 
ALTER COLUMN updatedAt SET DEFAULT CURRENT_TIMESTAMP;



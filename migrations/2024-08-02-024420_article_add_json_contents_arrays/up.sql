-- Your SQL goes here
ALTER TABLE articles 
ALTER COLUMN content TYPE JSONB[] USING ARRAY[content::JSONB];


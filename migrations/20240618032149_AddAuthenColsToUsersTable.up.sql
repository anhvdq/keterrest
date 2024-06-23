-- Add up migration script here
-- This script may fail due to existing users data
ALTER TABLE users 
    ADD COLUMN email VARCHAR NOT NULL, 
    ADD COLUMN password VARCHAR NOT NULL;

ALTER TABLE users 
    ADD CONSTRAINT email_is_unique UNIQUE (email);
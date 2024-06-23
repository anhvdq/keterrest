-- Add down migration script here
ALTER TABLE users 
    DROP CONSTRAINT email_is_unique;

ALTER TABLE users 
    DROP COLUMN email, 
    DROP COLUMN password;
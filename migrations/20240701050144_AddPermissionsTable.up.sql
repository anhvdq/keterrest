-- Add up migration script here
-- Create permissions table
CREATE TABLE permissions (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL
);

ALTER TABLE permissions 
    ADD CONSTRAINT permission_is_unique UNIQUE (name);

INSERT INTO permissions(name) VALUES ('user.create'), ('user.read'), ('user.update'), ('user.delete');

-- Create user_permissions table
CREATE TABLE users_permissions (
    id SERIAL PRIMARY KEY,
    user_id INTEGER NOT NULL,
    permission_id INTEGER NOT NULL
);

ALTER TABLE users_permissions
    ADD CONSTRAINT user_permission_is_unique UNIQUE (user_id, permission_id);

ALTER TABLE users_permissions
    ADD CONSTRAINT fk_users_permissions_to_users
    FOREIGN KEY (user_id) 
    REFERENCES users (id),
    ADD CONSTRAINT fk_users_permissions_to_permissions
    FOREIGN KEY (permission_id) 
    REFERENCES permissions (id);

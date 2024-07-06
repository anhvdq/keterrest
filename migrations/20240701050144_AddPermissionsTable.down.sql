-- Add down migration script here
ALTER TABLE users_permissions 
    DROP CONSTRAINT fk_users_permissions_to_users,
    DROP CONSTRAINT fk_users_permissions_to_permissions;

DROP TABLE users_permissions;

ALTER TABLE permissions 
    DROP CONSTRAINT permission_is_unique;

DROP TABLE permissions;
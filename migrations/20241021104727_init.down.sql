-- Add down migration script here
DROP TABLE IF EXISTS users CASCADE ;
DROP TABLE IF EXISTS token CASCADE ;
DROP TABLE IF EXISTS memories CASCADE ;
DROP TABLE IF EXISTS shared_memory_user_relationship CASCADE ;
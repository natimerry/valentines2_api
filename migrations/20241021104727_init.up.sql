-- Add up migration script here
CREATE TABLE IF NOT EXISTS users(
    username VARCHAR(255) PRIMARY KEY UNIQUE ,
    email VARCHAR(255) UNIQUE ,
    hashed_pass CHAR(128)
);

CREATE INDEX IF NOT EXISTS _user_email_idx ON users(email);

CREATE TABLE IF NOT EXISTS token(
    id uuid PRIMARY KEY,
    expiry timestamp,
    last_issue timestamp,
    username VARCHAR(255) references users(username)
);

CREATE TABLE IF NOT EXISTS memories(
    id uuid PRIMARY KEY,
    title TEXT,
    content TEXT,
    author VARCHAR(255) REFERENCES users(username)
);

CREATE TABLE shared_memory_user_relationship(
    id BIGSERIAL PRIMARY KEY,
    memory uuid REFERENCES memories(id),
    author VARCHAR(255) references users(username),
    shared_with VARCHAR(255) references users(username)
);

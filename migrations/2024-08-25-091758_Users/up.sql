-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users (
    id SERIAL PRIMARY KEY,
    username VARCHAR NOT NULL UNIQUE,
    passkey VARCHAR NOT NULL
);

CREATE TABLE IF NOT EXISTS sessions (
    user_id integer REFERENCES users(id) ON DELETE CASCADE,
    session_token BYTEA PRIMARY KEY
)
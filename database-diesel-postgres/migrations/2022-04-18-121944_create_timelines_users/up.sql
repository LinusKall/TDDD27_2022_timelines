-- Your SQL goes here

-- enum type for describing relation between a timeline and a user.
CREATE TYPE user_role AS ENUM ('owner', 'moderator', 'subscriber');

-- TODO: Add color.
CREATE TABLE timelines_users (
    id SERIAL PRIMARY KEY,
    timeline_id SERIAL NOT NULL REFERENCES timelines (id),
    user_id SERIAL NOT NULL REFERENCES users (id),
    relation user_role NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
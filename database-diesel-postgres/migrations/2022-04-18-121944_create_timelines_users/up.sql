-- Your SQL goes here

-- enum type for describing relation between a timeline and a user.
CREATE TYPE clearance AS ENUM ('owner', 'moderator', 'subscriber');

-- TODO: Add color.
CREATE TABLE timelines_users (
    timeline_id SERIAL NOT NULL REFERENCES timelines (id),
    user_id SERIAL NOT NULL REFERENCES users (id),
    relation clearance NOT NULL,
    color TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (timeline_id, user_id)
);
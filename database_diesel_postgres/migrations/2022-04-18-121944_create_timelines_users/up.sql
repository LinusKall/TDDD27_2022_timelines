-- Your SQL goes here

-- enum type for describing relation between a timeline and a user.
CREATE TYPE clearance_mapping AS ENUM ('OWNER', 'MODERATOR', 'SUBSCRIBER');

CREATE TABLE timelines_users (
    id SERIAL PRIMARY KEY,
    timeline_id SERIAL NOT NULL REFERENCES timelines (id),
    user_id SERIAL NOT NULL REFERENCES users (id),
    relation clearance_mapping NOT NULL,
    color TEXT NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
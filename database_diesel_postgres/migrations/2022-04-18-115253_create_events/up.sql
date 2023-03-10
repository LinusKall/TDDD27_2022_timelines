-- Your SQL goes here

CREATE TABLE events (
    id SERIAL PRIMARY KEY,
    timeline_id SERIAL NOT NULL REFERENCES timelines (id),
    title TEXT NOT NULL,
    body TEXT,
    start_time TIMESTAMP NOT NULL,
    end_time TIMESTAMP NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP
);
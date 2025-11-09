CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS joys (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    point GEOGRAPHY(Point, 4326),
    frustration TEXT NOT NULL,
    context TEXT NOT NULL,
    joy TEXT NOT NULL,
    created BIGINT NOT NULL
);

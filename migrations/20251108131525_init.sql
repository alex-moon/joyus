CREATE EXTENSION IF NOT EXISTS postgis;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    point GEOGRAPHY(Point, 4326)
);

CREATE TABLE IF NOT EXISTS joys (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    created TIMESTAMPTZ NOT NULL,
    point GEOGRAPHY(Point, 4326),
    frustration TEXT NOT NULL,
    context TEXT NOT NULL,
    joy TEXT NOT NULL
);

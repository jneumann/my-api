CREATE TABLE IF NOT EXISTS posts
(
    id SERIAL PRIMARY KEY NOT NULL,
    name VARCHAR(255),
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc'),
    published boolean DEFAULT false
)

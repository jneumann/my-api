CREATE TABLE IF NOT EXISTS vehicles
(
    id SERIAL PRIMARY KEY NOT NULL,
    vehicle INTEGER,
    day date,
    cost NUMERIC(5, 2),
    quantity NUMERIC(4, 1),
    miles NUMERIC(5, 1),
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc')
)

CREATE TABLE IF NOT EXISTS vehicles
(
    id SERIAL PRIMARY KEY NOT NULL,
    make VARCHAR(255),
    model VARCHAR(255),
    vin VARCHAR(255),
    color VARCHAR(255),
    starting_odometer NUMERIC(8, 1),
    current_odometer NUMERIC(8, 1),
    created_at timestamp with time zone DEFAULT (now() at time zone 'utc')
)

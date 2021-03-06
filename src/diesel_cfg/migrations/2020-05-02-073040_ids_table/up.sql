CREATE TABLE identifications (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    course VARCHAR NOT NULL,
    valid_from DATE,
    valid_till DATE,
    institution VARCHAR NOT NULL,
    campus VARCHAR NOT NULL, /*institution location*/
    location_name VARCHAR NOT NULL,
    location_point POINT, /*lat/longitude -> Use this with map api*/
    picture VARCHAR,
    posted_by INTEGER,
    is_found BOOLEAN NOT NULL DEFAULT FALSE,
    created_at timestamp without time zone not null default (now() at time zone 'utc'),
    updated_at timestamp without time zone not null default (now() at time zone 'utc')
);

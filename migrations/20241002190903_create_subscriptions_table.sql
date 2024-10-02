CREATE TABLE subscriptions (
    id uuid NOT NULL,
    PRIMARY KEY (id),
    email text NOT NULL UNIQUE,
    name text NOT NULL,
    created_at timestamptz NOT NULL
);
CREATE TABLE users (
    id UUID PRIMARY KEY,
    email TEXT NOT NULL,
    password TEXT NOT NULL,
    permissions TEXT[] NOT NULL
);

CREATE TABLE "users" (
    uuid VARCHAR PRIMARY KEY,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,

    permission permission default 'user',
    tokenversion bigint default 1,

    issued_at bigint DEFAULT EXTRACT(EPOCH FROM NOW())
);

CREATE TABLE "new_users" (
    uuid VARCHAR PRIMARY KEY,
    username VARCHAR NOT NULL,
    email VARCHAR NOT NULL,
    password VARCHAR NOT NULL,
    code VARCHAR NOT NULL
);
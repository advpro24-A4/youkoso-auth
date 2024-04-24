CREATE TABLE users
(
    id       VARCHAR(255) NOT NULL PRIMARY KEY,
    role     VARCHAR(31),
    email    VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL
);

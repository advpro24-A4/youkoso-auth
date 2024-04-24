CREATE TYPE user_role  AS ENUM ('ADMIN', 'CUSTOMER');

CREATE TABLE users
(
    id       VARCHAR(255) NOT NULL PRIMARY KEY,
    role     user_role,
    email    VARCHAR(255) NOT NULL UNIQUE,
    name VARCHAR(255),
    username VARCHAR(255) UNIQUE,
    password VARCHAR(255) NOT NULL,
    address VARCHAR(255),
    birth_date DATE,
    phone_number VARCHAR(255)
);

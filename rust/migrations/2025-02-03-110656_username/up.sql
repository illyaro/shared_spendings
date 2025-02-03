-- Your SQL goes here
CREATE TABLE username (
    id          VARCHAR(21) CONSTRAINT user_primary_key PRIMARY KEY,
    email       VARCHAR(100) NOT NULL,
    picture     VARCHAR(150) NOT NULL,
    user_name   VARCHAR(100) NOT NULL
)
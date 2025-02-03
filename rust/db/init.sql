CREATE TABLE username (
    id          VARCHAR(21) CONSTRAINT user_primary_key PRIMARY KEY,
    email       VARCHAR(100) NOT NULL,
    picture     VARCHAR(150) NOT NULL,
    user_name   VARCHAR(100) NOT NULL
);

CREATE TABLE record (
    id          BIGSERIAL CONSTRAINT record_primary_key PRIMARY KEY,
    user_id     VARCHAR(21) NOT NULL,
    amount      double precision NOT NULL,
    dt          TIMESTAMP NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY (user_id)
        REFERENCES username(id)
        ON DELETE CASCADE
);
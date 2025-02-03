CREATE TABLE record (
    id          BIGSERIAL CONSTRAINT record_primary_key PRIMARY KEY,
    user_id     VARCHAR(21) NOT NULL,
    amount      double precision NOT NULL,
    dt          TIMESTAMP NOT NULL,
    CONSTRAINT fk_user
        FOREIGN KEY (user_id)
        REFERENCES username(id)
        ON DELETE CASCADE
)

-- Add up migration script here
CREATE TABLE IF NOT EXISTS token_owner (
    token_id INT NOT NULL PRIMARY KEY,
    owner_id varchar(255) NOT NULL,
    block_number BIGINT NOT NULL
);

-- Add up migration script here
CREATE TABLE IF NOT EXISTS accounts (
  address VARCHAR(42) NOT NULL PRIMARY KEY,
  balance BIGINT NOT NULL  DEFAULT 0,
  block_number BIGINT NOT NULL,
  created_at BIGINT NOT NULL DEFAULT 0,
  updated_at BIGINT NOT NULL DEFAULT 0
);

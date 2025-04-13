-- Add up migration script here
CREATE TABLE IF NOT EXISTS block_offsets (
  address VARCHAR(255) NOT NULL PRIMARY KEY,
  current_offset bigint NOT NULL default 0
);

-- Your SQL goes here
CREATE TABLE products (
  id INTEGER PRIMARY KEY NOT NULL,
  name VARCHAR NOT NULL,
  cost DOUBLE NOT NULL,
  active BOOLEAN NOT NULL DEFAULT 0 --Sqlite does not have a Boolean value
)
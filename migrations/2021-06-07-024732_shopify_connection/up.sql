-- Your SQL goes here
CREATE TABLE shopifyConnection (
  id SERIAL PRIMARY KEY,
  shop VARCHAR NOT NULL,
  hmac VARCHAR NOT NULL,
  access_token VARCHAR,
  created_at TIMESTAMP NOT NULL,
  updated_at TIMESTAMP,
  deleted_at TIMESTAMP,
  active BOOLEAN NOT NULL
);

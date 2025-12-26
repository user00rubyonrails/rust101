-- Your SQL goes here
CREATE TABLE other_to_do_items (
  id SERIAL PRIMARY KEY,
  title VARCHAR NOT NULL,
  status VARCHAR NOT NULL
)
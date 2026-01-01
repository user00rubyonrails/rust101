-- Your SQL goes here
Alter table to_do ADD CONSTRAINT uc_item UNIQUE (title, user_id);
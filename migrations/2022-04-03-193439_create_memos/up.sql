-- Your SQL goes here

CREATE TABLE memos (
	id SERIAL PRIMARY KEY,
	name VARCHAR NOT NULL,
	body TEXT NOT NULL
);
CREATE SCHEMA IF NOT EXISTS import;
DROP TABLE IF EXISTS import.master_plan;
CREATE TABLE import.master_plan(
	start_time_utc TEXT,
	duration TEXT,
	date TEXT,
	team TEXT,
	spass_type TEXT,
	target TEXT,
	request_name TEXT,
	library_definition TEXT,
	title TEXT,
	description TEXT
);
COPY import.master_plan FROM '/docker-entrypoint-initdb.d/master_plan.csv' DELIMITER ',' CSV HEADER;

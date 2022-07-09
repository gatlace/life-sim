DROP SCHEMA if exists life_sim cascade;

-- DROP SCHEMA life_sim;

CREATE SCHEMA life_sim AUTHORIZATION postgres;
-- life_sim.choices definition

-- Drop table

-- DROP TABLE life_sim.choices;

CREATE TABLE life_sim.choices (
	id uuid NOT NULL,
	"name" varchar NOT NULL,
	min_age int4 NOT NULL,
	max_age int4 NOT NULL,
	CONSTRAINT choices_pk PRIMARY KEY (id)
);


-- life_sim.people definition

-- Drop table

-- DROP TABLE life_sim.people;

CREATE TABLE life_sim.people (
	id uuid NOT NULL,
	first_name varchar NOT NULL,
	last_name varchar NOT NULL,
	"money" int4 NOT NULL,
	credit int4 NULL,
	age int4 NOT NULL,
	CONSTRAINT people_pk PRIMARY KEY (id)
);


-- life_sim.decisions definition

-- Drop table

-- DROP TABLE life_sim.decisions;

CREATE TABLE life_sim.decisions (
	id uuid NOT NULL,
	choice_id uuid NOT NULL,
	description varchar NOT NULL,
	CONSTRAINT decisions_pk PRIMARY KEY (id),
	CONSTRAINT decisions_fk FOREIGN KEY (choice_id) REFERENCES life_sim.choices(id)
);


-- life_sim.effects definition

-- Drop table

-- DROP TABLE life_sim.effects;

CREATE TABLE life_sim.effects (
	id uuid NOT NULL,
	decision_id uuid NOT NULL,
	property varchar NOT NULL,
	value varchar NOT NULL,
	duration int4 NOT NULL,
	CONSTRAINT effects_pk PRIMARY KEY (id),
	CONSTRAINT effects_fk FOREIGN KEY (decision_id) REFERENCES life_sim.decisions(id)
);


-- life_sim.person_effect_links definition

-- Drop table

-- DROP TABLE life_sim.person_effect_links;

CREATE TABLE life_sim.person_effect_links (
	id uuid NOT NULL,
	person_id uuid NOT NULL,
	effect_id uuid NOT NULL,
	time_left int4 NOT NULL,
	CONSTRAINT person_effect_links_pk PRIMARY KEY (id),
	CONSTRAINT person_effect_links_fk FOREIGN KEY (person_id) REFERENCES life_sim.people(id) ON DELETE CASCADE ON UPDATE CASCADE,
	CONSTRAINT person_effect_links_fk_1 FOREIGN KEY (effect_id) REFERENCES life_sim.effects(id) ON DELETE CASCADE ON UPDATE CASCADE
);
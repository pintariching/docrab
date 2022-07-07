CREATE TABLE document_type 
(
	id bigserial PRIMARY KEY,
	label character varying(128) NOT NULL
);

CREATE TABLE tag
(
	id bigserial PRIMARY KEY,
	label character varying(128) NOT NULL,
	color character varying(8) NOT NULL
);

CREATE TABLE document
(
	id bigserial PRIMARY KEY,
	label character varying(128) NOT NULL,
	document_type_id bigint NOT NULL,
	CONSTRAINT document_document_type_fkey FOREIGN KEY (document_type_id)
		REFERENCES document_type (id) MATCH SIMPLE
		ON UPDATE CASCADE
		ON DELETE RESTRICT
);

CREATE TABLE document_file
(
	id bigserial PRIMARY KEY,
	label character varying(128) NOT NULL,
	document_id bigint NOT NULL,
	version character varying(128) NOT NULL,
	filename character varying(128) NOT NULL,
	CONSTRAINT document_document_file_fkey FOREIGN KEY (document_id)
		REFERENCES document (id) MATCH SIMPLE
		ON UPDATE CASCADE
		ON DELETE RESTRICT
);

CREATE TABLE document_tag
(
	id bigserial PRIMARY KEY,
	tag_id bigint NOT NULL,
	document_id bigint NOT NULL,
	CONSTRAINT document_tag_tag_fkey FOREIGN KEY (tag_id)
		REFERENCES tag (id) MATCH SIMPLE
		ON UPDATE CASCADE
		ON DELETE CASCADE,
	CONSTRAINT document_tag_document_fkey FOREIGN KEY (document_id)
		REFERENCES document (id) MATCH SIMPLE
		ON UPDATE CASCADE
		ON DELETE CASCADE
);
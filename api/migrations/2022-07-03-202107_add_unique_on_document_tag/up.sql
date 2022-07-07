ALTER TABLE document_tag ADD CONSTRAINT
	document_id_tag_id_unique UNIQUE (document_id, tag_id);
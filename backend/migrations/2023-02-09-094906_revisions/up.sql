CREATE TABLE IF NOT EXISTS ticket_revisions (
	revision_id UUID PRIMARY KEY,
	ticket_id INTEGER NOT NULL,
	description TEXT NOT NULL,
	updated_by UUID,
	updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT fk_ticket_revisions_ticket
		FOREIGN KEY (ticket_id)
		REFERENCES tickets (ticket_id)
		ON DELETE CASCADE,
	CONSTRAINT fk_ticket_revisions_user
		FOREIGN KEY (updated_by)
		REFERENCES users (user_id)
		ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS document_revisions (
	revision_id UUID PRIMARY KEY,
	document_id UUID NOT NULL,
	content TEXT NOT NULL,
	updated_by UUID,
	updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT fk_document_revisions_document
		FOREIGN KEY (document_id)
		REFERENCES documents (document_id)
		ON DELETE CASCADE,
	CONSTRAINT fk_document_revisions_user
		FOREIGN KEY (updated_by)
		REFERENCES users (user_id)
		ON DELETE SET NULL
);
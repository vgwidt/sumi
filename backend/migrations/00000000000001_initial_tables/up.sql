CREATE TABLE IF NOT EXISTS users (
	user_id UUID PRIMARY KEY,
	username TEXT NOT NULL,
	display_name TEXT NOT NULL,
	email TEXT NOT NULL,
	UNIQUE(username),
	UNIQUE(email),
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	access TEXT NOT NULL DEFAULT '1',
	password_hash TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS contacts
(
	contact_id UUID PRIMARY KEY,
	display_name TEXT NOT NULL,
	email TEXT NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	UNIQUE(email)
);

CREATE TABLE IF NOT EXISTS tickets
(
	ticket_id SERIAL PRIMARY KEY,
	assignee UUID,
	contact UUID,
	title TEXT NOT NULL,
	description TEXT NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	due_date TIMESTAMP,
	priority TEXT NOT NULL,
	status TEXT NOT NULL,
	CONSTRAINT fk_ticket_assignee
		FOREIGN KEY (assignee)
		REFERENCES users (user_id)
		ON DELETE SET NULL,
	CONSTRAINT fk_ticket_contact
		FOREIGN KEY (contact)
		REFERENCES contacts (contact_id)
		ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS notes
(
	note_id UUID PRIMARY KEY,
	ticket INTEGER NOT NULL,
	owner UUID,
	text TEXT NOT NULL,
	time INTEGER NOT NULL DEFAULT 0,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT fk_note_ticket
		FOREIGN KEY (ticket)
		REFERENCES tickets (ticket_id)
		ON DELETE CASCADE,
	CONSTRAINT fk_note_user
		FOREIGN KEY (owner)
		REFERENCES users (user_id)
		ON DELETE SET NULL
);

-- add resolution fields to tickets
ALTER TABLE tickets ADD COLUMN resolution UUID DEFAULT NULL;
ALTER TABLE tickets ADD CONSTRAINT fk_ticket_resolution FOREIGN KEY (resolution) REFERENCES notes (note_id) ON DELETE SET NULL;

-- default admin
INSERT INTO users (user_id, username, display_name, email, password_hash)
VALUES ('ddf8994f-d522-4659-8d02-c1d479057be6',
   'admin',
   'Administrator',
   'admin@localhost',
   '$argon2id$v=19$m=15000,t=2,p=1$eTU5WXBRdnk0S0l2VkFZVQ$mGkOYhkATalY2D6eqd3teA'
) ON CONFLICT DO NOTHING;

CREATE TABLE IF NOT EXISTS user_preferences
(
	user_id UUID PRIMARY KEY,
	theme TEXT,
	locale TEXT,
	timezone TEXT,
	CONSTRAINT fk_user_preferences_user
		FOREIGN KEY (user_id)
		REFERENCES users (user_id)
		ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS documents
(
	document_id UUID PRIMARY KEY,
	parent_id UUID,
	url TEXT NOT NULL,
	title TEXT NOT NULL,
	content TEXT NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	created_by UUID,
	updated_by UUID,
	archived BOOLEAN NOT NULL DEFAULT FALSE,
	CONSTRAINT fk_documents_parent
		FOREIGN KEY (parent_id)
		REFERENCES documents (document_id)
		ON DELETE SET NULL,
	CONSTRAINT fk_documents_created_by
		FOREIGN KEY (created_by)
		REFERENCES users (user_id)
		ON DELETE SET NULL,
	CONSTRAINT fk_documents_updated_by
		FOREIGN KEY (updated_by)
		REFERENCES users (user_id)
		ON DELETE SET NULL,
	CHECK (document_id <> parent_id),
	UNIQUE(url)
);

CREATE TABLE IF NOT EXISTS comments
(
	comment_id UUID PRIMARY KEY,
	document_id UUID NOT NULL,
	author UUID,
	text TEXT NOT NULL,
	created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
	CONSTRAINT fk_comments_document
		FOREIGN KEY (document_id)
		REFERENCES documents (document_id)
		ON DELETE CASCADE,
	CONSTRAINT fk_comments_user
		FOREIGN KEY (author)
		REFERENCES users (user_id)
		ON DELETE SET NULL
);

INSERT INTO user_preferences (user_id) SELECT user_id FROM users;
ALTER TABLE tickets DROP COLUMN revision_by;
ALTER TABLE tickets DROP COLUMN revision;
ALTER TABLE tickets DROP COLUMN updated_by;
ALTER TABLE tickets DROP COLUMN created_by;
DROP TABLE ticket_events;
DROP TABLE document_revisions;
DROP TABLE ticket_revisions;
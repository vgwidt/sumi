ALTER TABLE tickets ADD COLUMN created_by UUID,
    ADD COLUMN updated_by UUID,
    ADD COLUMN revision TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ADD COLUMN revision_by UUID;

UPDATE tickets SET revision = updated_at;
ALTER TABLE tickets ALTER COLUMN revision DROP DEFAULT;
ALTER TABLE tokens ADD COLUMN status VARCHAR(255) NOT NULL;
ALTER TABLE tokens ADD CONSTRAINT check_status CHECK (status IN ('ACTIVE', 'INACTIVE', 'REVOKED'));

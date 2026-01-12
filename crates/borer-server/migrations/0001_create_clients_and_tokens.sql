-- Enable uuid generation
CREATE
EXTENSION IF NOT EXISTS "pgcrypto";

CREATE TABLE clients
(
    id         UUID PRIMARY KEY                  DEFAULT gen_random_uuid(),
    name       TEXT,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now()
);

CREATE TABLE tokens
(
    token        TEXT PRIMARY KEY,
    client_id    UUID                     NOT NULL REFERENCES clients (id) ON DELETE CASCADE,
    revoked      BOOLEAN                  NOT NULL DEFAULT false,
    created_at   TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT now(),
    last_used_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_tokens_client_id ON tokens (client_id);
CREATE INDEX idx_tokens_revoked ON tokens (revoked);

-- Add migration script here
CREATE TABLE rankings (
    id uuid NOT NULL,
    account_id VARCHAR(255) NOT NULL,
    normal_mmr BIGINT NOT NULL,
    ranked_mmr BIGINT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ DEFAULT current_timestamp,
    CONSTRAINT rankings_pkey PRIMARY KEY (id)
);

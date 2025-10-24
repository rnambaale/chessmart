-- Add migration script here
CREATE TABLE mmr_changes (
    id uuid NOT NULL,
    account_id VARCHAR(255) NOT NULL,
    game_id VARCHAR(255) NOT NULL,
    game_type VARCHAR(255) NOT NULL,
    mmr_change BIGINT NOT NULL,
    is_ranked BOOLEAN NOT NULL,
    created_at TIMESTAMPTZ DEFAULT current_timestamp,
    updated_at TIMESTAMPTZ DEFAULT current_timestamp,
    CONSTRAINT mmr_changes_pkey PRIMARY KEY (id)
);

-- // Unique index to prevent at db level multiple mmr change records for the same game+account
CREATE UNIQUE INDEX idx_mmr_change_game_id_account_id_unique_index
ON mmr_changes
USING btree (game_id, account_id);

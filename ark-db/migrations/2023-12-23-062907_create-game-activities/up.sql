-- Your SQL goes here

 CREATE TABLE coinflip_game_activities (
                id BIGSERIAL PRIMARY KEY,
                game_id BIGINT NOT NULL,
                chain_id BIGINT NOT NULL,
                trigger_public_address VARCHAR NOT NULL,
                kind VARCHAR NOT NULL,
                data JSON DEFAULT '{}',
                occurred_at BIGINT,
                transaction_hash VARCHAR
            );
-- Your SQL goes here
CREATE TABLE coinflip_chain_currencies (
                id SERIAL PRIMARY KEY,
                chain_id BIGINT NOT NULL,
                currency_symbol VARCHAR NOT NULL,
                unit_usd_price VARCHAR NOT NULL
            );


CREATE UNIQUE INDEX unique_coinflip_chain_currency_symbol ON coinflip_chain_currencies(chain_id, currency_symbol);
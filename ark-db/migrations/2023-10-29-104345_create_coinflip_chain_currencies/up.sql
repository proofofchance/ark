-- Your SQL goes here
CREATE TABLE coinflip_chain_currencies (
                id SERIAL PRIMARY KEY,
                chain_id INTEGER NOT NULL,
                currency_symbol VARCHAR NOT NULL,
                unit_price_in_usd VARCHAR NOT NULL
            )   

-- Your SQL goes here

 CREATE TABLE ark_total_paid_out_reports (
                id BIGSERIAL PRIMARY KEY,
                amount VARCHAR NOT NULL,
                reported_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
            );
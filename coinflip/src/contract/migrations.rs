use chaindexing::ContractStateMigrations;

pub struct GamesMigrations;

impl ContractStateMigrations for GamesMigrations {
    fn migrations(&self) -> Vec<&'static str> {
        vec![
            "CREATE TABLE IF NOT EXISTS coinflip_games (
                id BIGINT NOT NULL,
                max_play_count INTEGER NOT NULL,
                expiry_timestamp BIGINT NOT NULL,
                creator_address VARCHAR NOT NULL,
                wager BIGINT NOT NULL
            )",
        ]
    }
}

mod game_created;
mod game_play_created;
mod game_play_proof_created;

pub use game_created::GameCreatedEventHandler;
pub use game_play_created::GamePlayCreatedEventHandler;
pub use game_play_proof_created::GamePlayProofCreatedEventHandler;

use super::states::GameActivity;
use chaindexing::{ContractState, EventContext};

pub async fn record_new_game_activity<'a>(
    game_id: u64,
    block_timestamp: u64,
    context: &EventContext<'a>,
) {
    GameActivity {
        game_id,
        block_timestamp,
    }
    .create(context)
    .await;
}

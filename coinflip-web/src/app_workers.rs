use self::index_evm_states::IndexEvmStates;

mod index_evm_states;

pub struct AppWorkers;

impl AppWorkers {
    pub fn start() {
        IndexEvmStates::start();
    }
}

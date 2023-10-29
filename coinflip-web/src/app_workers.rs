mod cache_chain_unit_currencies_in_usd;
mod index_evm_states;

pub struct AppWorkers;

impl AppWorkers {
    pub fn start() {
        index_evm_states::start();
    }
}

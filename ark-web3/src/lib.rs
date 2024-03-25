pub mod chain_explorers;
pub mod chains;
pub mod json_rpcs;
pub mod wallets;

pub const GWEI: u64 = 1000000000;
pub const GWEI_F64: f64 = 1000000000.0;

pub const CHAIN_AGNOSTIC_MAX_GAS_PRICE: u64 = 100 * GWEI;
pub const CHAIN_AGNOSTIC_MAX_GAS_PRICE_F64: f64 = 100.0 * GWEI_F64;

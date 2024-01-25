use diesel::deserialize::Queryable;

#[derive(Clone, Debug, Queryable)]
#[diesel(table_name = ark_wallets)]
pub struct Wallet {
    pub id: i64,
    pub chain_id: i64,
    pub owner_address: String,
    pub balance: String,
}

impl Wallet {
    pub fn get_balance_ether(&self) -> f64 {
        self.balance.parse().unwrap()
    }
}

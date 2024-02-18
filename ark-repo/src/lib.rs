use ark::wallets::Wallet;

use ark_db::DBConn;

use ark_web3::chains::{ChainCurrency, UnsavedChainCurrency};

use diesel::{upsert::excluded, ExpressionMethods, OptionalExtension, QueryDsl};
use diesel_async::RunQueryDsl;

pub async fn create_or_update_chain_currencies<'a>(
    conn: &mut DBConn<'a>,
    chain_currencies: &Vec<UnsavedChainCurrency>,
) {
    use ark_db::schema::ark_chain_currencies::dsl::*;

    diesel::insert_into(ark_chain_currencies)
        .values(chain_currencies)
        .on_conflict((chain_id, currency_symbol))
        .do_update()
        .set(unit_usd_price.eq(excluded(unit_usd_price)))
        .execute(conn)
        .await
        .unwrap();
}

pub async fn get_chain_currencies<'a>(
    conn: &mut DBConn<'a>,
    chain_ids: &Vec<i64>,
) -> Vec<ChainCurrency> {
    use ark_db::schema::ark_chain_currencies::dsl::*;

    ark_chain_currencies
        .filter(chain_id.eq_any(chain_ids))
        .load(conn)
        .await
        .unwrap()
}

pub async fn get_chain_currency<'a>(
    conn: &mut DBConn<'a>,
    chain_id_: i64,
) -> Option<ChainCurrency> {
    use ark_db::schema::ark_chain_currencies::dsl::*;

    ark_chain_currencies
        .filter(chain_id.eq(chain_id_))
        .first(conn)
        .await
        .optional()
        .unwrap()
}

pub async fn get_wallet<'a>(
    conn: &mut DBConn<'a>,
    owner_address_: &str,
    chain_id_: i64,
) -> Option<Wallet> {
    use ark_db::schema::ark_wallets::dsl::*;

    ark_wallets
        .filter(owner_address.eq(owner_address_.to_lowercase()))
        .filter(chain_id.eq(chain_id_))
        .first(conn)
        .await
        .optional()
        .unwrap()
}

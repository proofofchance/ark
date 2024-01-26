use ark_web_app::AppState;
use axum::extract::{Json, Path, State};
use http::StatusCode;

use serde::{Deserialize, Serialize};

use crate::handlers;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct WalletResponse {
    pub owner_address: String,
    pub balance: String,
    pub balance_usd: String,
}

pub async fn get_wallet(
    State(app_state): State<AppState>,
    Path((public_address, chain_id)): Path<(String, u64)>,
) -> Result<Json<WalletResponse>, handlers::Error> {
    let chain_id = chain_id as i64;
    let mut conn = handlers::new_conn(app_state.db_pool).await?;

    let chain_currency = ark_repo::get_chain_currency(&mut conn, chain_id).await.unwrap();
    let maybe_wallet = ark_repo::get_wallet(&mut conn, &public_address, chain_id).await;

    match maybe_wallet {
        Some(wallet) => {
            let balance_ether = wallet.get_balance_ether();

            Ok(Json(WalletResponse {
                owner_address: wallet.owner_address,
                balance: to_2dp(balance_ether),
                balance_usd: to_2dp(chain_currency.convert_to_usd(balance_ether)),
            }))
        }
        None => Err((StatusCode::NOT_FOUND, "Wallet not found".to_string())),
    }
}

fn to_2dp(value: f64) -> String {
    format!("{:.02}", value.abs())
}

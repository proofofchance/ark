use std::collections::HashMap;

const BASE_URL: &str = "https://min-api.cryptocompare.com";

#[derive(Debug)]
pub enum CryptoCompareError {
    Generic(String),
}

pub async fn get_unit_price_in_usd(chain_currency: &str) -> Result<f32, CryptoCompareError> {
    const USD_CURRENCY: &'static str = "USD";

    let from_symbol = chain_currency;
    let to_symbols = vec![USD_CURRENCY].join(",");
    let endpoint_url = format!("{BASE_URL}/data/price?fsym={from_symbol}&tsyms={to_symbols}");

    let response = reqwest::get(&endpoint_url)
        .await
        .map_err(|err| CryptoCompareError::Generic(err.to_string()))?;

    let response_json = response.json::<HashMap<String, f32>>().await.unwrap();

    Ok(response_json.get(USD_CURRENCY).cloned().unwrap())
}

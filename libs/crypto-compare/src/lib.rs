use std::collections::HashMap;

const BASE_URL: &str = "https://min-api.cryptocompare.com";

#[derive(Debug)]
pub enum CryptoCompareError {
    Generic(String),
}

pub async fn get_unit_price_in_usd(
    crypto_currency_symbol: &str,
) -> Result<f32, CryptoCompareError> {
    const USD_SYMBOL: &'static str = "USD";

    let from_symbol = crypto_currency_symbol;
    let to_symbols = vec![USD_SYMBOL].join(",");
    let endpoint_url = format!("{BASE_URL}/data/price?fsym={from_symbol}&tsyms={to_symbols}");

    let response = reqwest::get(&endpoint_url)
        .await
        .map_err(|err| CryptoCompareError::Generic(err.to_string()))?;

    let response_json = response
        .json::<HashMap<String, f32>>()
        .await
        .map_err(|e| CryptoCompareError::Generic(e.to_string()))?;

    Ok(response_json.get(USD_SYMBOL).cloned().unwrap())
}

pub async fn get_unit_prices_in_usd(
    crypto_currency_symbols: &Vec<&str>,
) -> Result<HashMap<String, f32>, CryptoCompareError> {
    let usd_prices_per_unit = get_usd_prices_per_unit(crypto_currency_symbols).await?;

    let unit_prices_in_usd: HashMap<String, f32> = usd_prices_per_unit
        .iter()
        .map(|(currency_symbol, usd_price_per_unit)| {
            (currency_symbol.to_string(), 1 as f32 / *usd_price_per_unit)
        })
        .collect();

    Ok(unit_prices_in_usd)
}

pub async fn get_usd_prices_per_unit(
    crypto_currency_symbols: &Vec<&str>,
) -> Result<HashMap<String, f32>, CryptoCompareError> {
    const USD_SYMBOL: &'static str = "USD";

    let from_symbol = USD_SYMBOL;
    let to_symbols = crypto_currency_symbols.join(",");
    let endpoint_url = format!("{BASE_URL}/data/price?fsym={from_symbol}&tsyms={to_symbols}");

    let response = reqwest::get(&endpoint_url)
        .await
        .map_err(|err| CryptoCompareError::Generic(err.to_string()))?;

    let response_json = response
        .json::<HashMap<String, f32>>()
        .await
        .map_err(|e| CryptoCompareError::Generic(e.to_string()))?;

    Ok(response_json)
}

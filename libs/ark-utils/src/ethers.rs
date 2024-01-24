use crate::strings;

pub fn convert_wei_to_ether(amount: &str) -> f64 {
    let amount = strings::truncate_string(amount, 10);

    let amount: f64 = amount.parse().unwrap();

    amount / (10 as f64).powf(8.0)
}

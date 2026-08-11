use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug)]
struct ExchangeResponse {
    rates: HashMap<String, f64>,
}

pub fn invoke_exchange(
    from: Option<String>,
    to: Option<String>,
    amount: Option<f64>,
) -> Result<(), Box<dyn std::error::Error>> {
    if let (Some(from), Some(to)) = (from, to) {
        let amt: f64 = amount.unwrap_or(1.0);
        let url: String =
            format!("https://api.frankfurter.dev/v1/latest?amount={amt}&from={from}&to={to}");
        let response: ExchangeResponse = reqwest::blocking::get(url)?.json()?;
        if let Some(rate) = response.rates.get(&to) {
            println!("{amt} {from} = {rate} {to}");
        }
        Ok(())
    } else {
        let response: HashMap<String, String> =
            reqwest::blocking::get("https://api.frankfurter.dev/v1/currencies")?.json()?;
        for (key, value) in response {
            println!("{key}: {value}");
        }
        Ok(())
    }
}

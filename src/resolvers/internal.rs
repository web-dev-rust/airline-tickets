use crate::boundaries::http_out::best_prices;
use crate::schema::{errors::InputError, model::web::BestPrices};

pub fn best_prices_info(
    departure: String,
    origin: String,
    destination: String,
) -> Result<BestPrices, InputError> {
    let best_prices_text = best_prices(departure, origin, destination)
        .unwrap()
        .text()
        .expect("Failed to parse best prices response");

    let best_prices: BestPrices = serde_json::from_str(&best_prices_text).unwrap();

    Ok(best_prices)
}

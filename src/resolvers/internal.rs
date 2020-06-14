use crate::boundaries::http_out::best_prices;
use crate::schema::{errors::GenericError, model::web::best_prices::BestPrices};

pub fn best_prices_info(
    departure: String,
    origin: String,
    destination: String,
) -> Result<BestPrices, GenericError> {
    let best_prices_text = best_prices(departure, origin, destination)?.text()?;
    let best_prices: BestPrices = serde_json::from_str(&best_prices_text)?;

    Ok(best_prices)
}

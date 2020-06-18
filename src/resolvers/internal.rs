use crate::boundaries::{
    http_out::{best_prices, recommendations},
    redis::redis_client};
use crate::schema::{errors::GenericError, model::web::{best_prices::BestPrices, recommendations::Recommendations}};
use redis::Commands;
use chrono::Utc;

pub fn best_prices_info(
    departure: String,
    origin: String,
    destination: String,
) -> Result<BestPrices, GenericError> {
    let mut con = redis_client()?.get_connection()?;
    let today = Utc::today().to_string();
    let redis_key = format!("bp{}:{}:{}:{}", &today, &departure, &origin, &destination);

    let best_prices_text = match con.get(&redis_key) {
        Ok(response) => response,
        Err(_) => {
            let _best_prices = best_prices(departure, origin, destination)?.text()?;
            let _: () = con.set(&redis_key, &_best_prices)?;
            _best_prices
        }
    };

    let best_prices: BestPrices = serde_json::from_str(&best_prices_text)?;

    Ok(best_prices)
}

pub fn recommendations_info(
    departure: String,
    origin: String,
    destination: String,
) -> Result<Recommendations, GenericError> {
    let mut con = redis_client()?.get_connection()?;
    let today = Utc::today().to_string();
    let redis_key = format!("r{}:{}:{}:{}", &today, &departure, &origin, &destination);

    let recommendations_text = match con.get(&redis_key) {
        Ok(response) => response,
        Err(_) => {
            let _recommendations = recommendations(departure, origin, destination)?.text()?;
            let _: () = con.set(&redis_key, &_recommendations)?;
            _recommendations
        }
    };
    
    let recommendations: Recommendations = serde_json::from_str(&recommendations_text)?;

    Ok(recommendations)
}
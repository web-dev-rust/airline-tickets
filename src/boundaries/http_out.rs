use reqwest::{blocking::Response, Result};

pub fn best_prices(departure: String, origin: String, destination: String) -> Result<Response> {
    let url =
        format!("https://bff.latam.com/ws/proxy/booking-webapp-bff/v1/public/revenue/bestprices/oneway?departure={}&origin={}&destination={}&cabin=Y&country=BR&language=PT&home=pt_br&adult=1&promoCode=",
                departure, origin, destination);
    reqwest::blocking::get(&url)
}

pub fn recommendations(departure: String, origin: String, destination: String) -> Result<Response> {
    let url =
        format!("https://bff.latam.com/ws/proxy/booking-webapp-bff/v1/public/revenue/recommendations/oneway?departure={}&origin={}&destination={}&cabin=Y&country=BR&language=PT&home=pt_br&adult=1&promoCode=",
                departure, origin, destination);
    reqwest::blocking::get(&url)
}
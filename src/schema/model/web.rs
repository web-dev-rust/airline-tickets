use juniper::GraphQLObject;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, GraphQLObject)]
#[serde(rename_all = "camelCase")]
pub struct BestPrices {
    itinerary: Itinerary,
    best_prices: Vec<BestPrice>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, GraphQLObject)]
#[serde(rename_all = "camelCase")]
pub struct Itinerary {
    date: String,
    origin_destinations: Vec<OriginDestination>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, GraphQLObject)]
pub struct OriginDestination {
    duration: i32,
    departure: AirportInfo,
    arrival: AirportInfo,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, GraphQLObject)]
pub struct AirportInfo {
    airport: String,
    city: String,
    country: String,
    timestamp: String,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, GraphQLObject)]
pub struct BestPrice {
    date: String,
    available: bool,
    price: Price,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, GraphQLObject)]
pub struct Price {
    amount: f64,
    currency: String,
}

pub mod best_prices {
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
}

pub mod recommendations {
    use juniper::GraphQLObject;
    use serde::{Deserialize, Serialize};

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, GraphQLObject)]
    pub struct Recommendations {
        data: Vec<Data>,
        status: Status,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, GraphQLObject)]
    pub struct Status {
        code: i32,
        message: String,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, GraphQLObject)]
    #[serde(rename_all = "camelCase")]
    pub struct Data {
        flights: Vec<Flight>,
        recommended_flight_code: String,
        currency: String,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, GraphQLObject)]
    #[serde(rename_all = "camelCase")]
    pub struct Flight {
        flight_code: String,
        arrival: Location,
        departure: Location,
        stops: i32,
        segments: Vec<Segment>,
        flight_duration: String,
        cabins: Vec<Cabin>,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, GraphQLObject)]
    #[serde(rename_all = "camelCase")]
    pub struct Location {
        airport_code: String,
        airport_name: String,
        city_code: String,
        city_name:String,
        country_code: String,
        date: String,
        date_time: String,
        time: Time,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, GraphQLObject)]
    pub  struct Time {
        stamp: String, 
        hours: String, 
        minutes: String, 
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, GraphQLObject)]
    #[serde(rename_all = "camelCase")]
    pub struct Segment {
        flight_code: String,
        flight_number: String,
        equipment: Info,
        airline: Airline,
        duration: String,
        departure: Location,
        arrival: Location,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, GraphQLObject)]
    pub struct Info {
        name: String,
        code: String,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, GraphQLObject)]
    pub struct Airline {
        marketing: Info,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, GraphQLObject)]
    #[serde(rename_all = "camelCase")]
    pub struct Cabin {
        code: String,
        display_price: f64,
        availability_count: i32,
        display_prices: DisplayPrice,
        fares: Vec<Fare>
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, GraphQLObject)]
    #[serde(rename_all = "camelCase")]
    pub struct DisplayPrice {
        slice: f64,
        whole_trip: f64,
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, GraphQLObject)]
    #[serde(rename_all = "camelCase")]
    pub struct Fare {
        code: String,
        category: String,
        fare_id: String,
        availability_count: i32,
        price: Price
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, GraphQLObject)]
    pub struct Price {
        adult: PriceInfo
    }

    #[derive(Serialize, Deserialize, Debug, PartialEq, Clone, GraphQLObject)]
    #[serde(rename_all = "camelCase")]
    pub struct PriceInfo {
        amount_without_tax: f64, 
        tax_and_fees: f64, 
        total: f64, 
    }
}
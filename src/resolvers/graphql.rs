use crate::core::error;
use crate::resolvers::internal::{best_prices_info, recommendations_info};
use crate::schema::{errors::{GenericError}, model::web::{best_prices::BestPrices, recommendations::Recommendations}};
use juniper::FieldResult;
use juniper::RootNode;

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn ping() -> FieldResult<String> {
        Ok(String::from("pong"))
    }

    fn bestPrices(
        departure: String,
        origin: String,
        destination: String,
    ) -> Result<BestPrices, GenericError> {
        error::iata_format(&origin, &destination)?;
        error::departure_date_format(&departure)?;
        let best_price = best_prices_info(departure, origin, destination)?;
        Ok(best_price)
    }

    fn recommendations(
        departure: String,
        origin: String,
        destination: String,
    ) -> Result<Recommendations, GenericError> {
        error::iata_format(&origin, &destination)?;
        error::departure_date_format(&departure)?;
        let recommendations = recommendations_info(departure, origin, destination)?;
        Ok(recommendations)
    }
}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {}

pub type Resolver = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_resolver() -> Resolver {
    Resolver::new(QueryRoot {}, MutationRoot {})
}

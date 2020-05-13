use crate::core::error;
use crate::resolvers::internal::best_prices_info;
use crate::schema::{errors::InputError, model::web::BestPrices};
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
    ) -> Result<BestPrices, InputError> {
        match (
            error::iata_format(&origin, &destination),
            error::departure_date_format(&departure),
        ) {
            (Err(e), Err(e2)) => Err(e),
            (Err(e), _) => Err(e),
            (_, Err(e)) => Err(e),
            _ => best_prices_info(departure, origin, destination),
        }
    }
}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {}

pub type Resolver = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_resolver() -> Resolver {
    Resolver::new(QueryRoot {}, MutationRoot {})
}

use crate::core::error;
use crate::schema::errors::InputError;
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
    ) -> Result<String, InputError> {
        match error::iata_format(&origin, &destination) {
            Err(e) => Err(e),
            Ok(_) => Ok(String::from("test")),
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

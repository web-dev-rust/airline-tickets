use juniper::FieldResult;
use juniper::RootNode;

pub struct QueryRoot;

#[juniper::object]
impl QueryRoot {
    fn ping() -> FieldResult<String> {
        Ok(String::from("pong"))
    }

    fn bestPrices(departure: String, origin: String, destinantion: String) -> FieldResult<String> {
         Ok(String::from("test"))
    }
}

pub struct MutationRoot;

#[juniper::object]
impl MutationRoot {}

pub type Resolver = RootNode<'static, QueryRoot, MutationRoot>;

pub fn create_resolver() -> Resolver {
    Resolver::new(QueryRoot {}, MutationRoot {})
}

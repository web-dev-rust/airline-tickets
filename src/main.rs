#[macro_use]
extern crate juniper;
extern crate serde_json;

use actix_web::{web, App, HttpServer, http::header};
use actix_cors::Cors;

mod boundaries;
mod resolvers;
mod schema;
mod core;
#[cfg(test)] mod test;

use crate::boundaries::web::routes;
use crate::resolvers::graphql::{create_resolver, Resolver};


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let resolvers: std::sync::Arc<Resolver> = std::sync::Arc::new(create_resolver());

    HttpServer::new(move || {
        App::new()
            .data(resolvers.clone())
            .wrap(
                Cors::new()
                    .allowed_origin("http://localhost:8080")
                    .allowed_origin("http://127.0.0.1:8080")
                    .allowed_origin("http://0.0.0.0:8080")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .max_age(3600)
                    .finish(),
            )
            .configure(routes)
            .default_service(web::to(|| async { "404" }))
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}

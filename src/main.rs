#[macro_use]
extern crate juniper;
extern crate serde_json;

use actix_web::{web, App, HttpServer};

mod handlers;
mod schemas;

use crate::handlers::routes;
use crate::schemas::{create_schema, Schema};


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let schema: std::sync::Arc<Schema> = std::sync::Arc::new(create_schema());

    HttpServer::new(move || {
        App::new()
            .data(schema.clone())
            .configure(routes)
            .default_service(web::to(|| async { "404" }))
    })
    .bind("127.0.0.1:4000")?
    .run()
    .await
}

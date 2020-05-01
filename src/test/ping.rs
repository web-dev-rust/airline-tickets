#[cfg(test)]
mod ping_readiness {
    use crate::handlers::routes;
    use crate::schemas::{create_schema, Schema};

    use actix_web::{test, App};
    use bytes::Bytes;

    #[actix_rt::test]
    async fn test_ping_pong() {
        let schema: std::sync::Arc<Schema> = std::sync::Arc::new(create_schema());

        let mut app =
            test::init_service(App::new().data(schema.clone()).configure(routes)).await;

        let req = test::TestRequest::post()
            .uri("/graphql")
            .header("Content-Type", "application/json")
            .set_payload("{\"query\": \"query ping { ping }\"}")
            .to_request();
        let resp = test::read_response(&mut app, req).await;

        assert_eq!(resp, Bytes::from_static(b"{\"data\":{\"ping\":\"pong\"}}"));
    }
}

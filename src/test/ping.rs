#[cfg(test)]
mod ping_readiness {
    use crate::boundaries::web::routes;
    use crate::resolvers::graphql::{create_resolver, Resolver};

    use actix_web::{test, App};
    use bytes::Bytes;

    #[actix_rt::test]
    async fn test_ping_pong() {
        let resolvers: std::sync::Arc<Resolver> = std::sync::Arc::new(create_resolver());

        let mut app =
            test::init_service(App::new().data(resolvers.clone()).configure(routes)).await;

        let req = test::TestRequest::post()
            .uri("/graphql")
            .header("Content-Type", "application/json")
            .set_payload("{\"query\": \"query ping { ping }\"}")
            .to_request();
        let resp = test::read_response(&mut app, req).await;

        assert_eq!(resp, Bytes::from_static(b"{\"data\":{\"ping\":\"pong\"}}"));
    }
}

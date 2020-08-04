# airline-tickets

## Decision History
1. GraphQL with Juniper
2. Queries make blocking::request to Latam API. (Juniper doesn't support ASYNC).
3. Cache everything with redis
4. CORS

#[cfg(test)]
mod test_routes {
    use axum::{Router, extract::Request, routing::get};
    use axum_test::TestServer;
    use http::{Method, StatusCode};

    #[tokio::test]
    async fn test_multiple_routers() {
        async fn route(method: Method) -> String {
            format!("Hello, {}", method)
        }

        let first = Router::new().route("/first", get(route));
        let second = Router::new().route("/second", get(route));

        let app = Router::new().merge(first).merge(second);

        let server = TestServer::new(app).unwrap();
        let response = server.get("/first").await;
        response.assert_status_ok();
        response.assert_text("Hello, GET");

        let response = server.get("/second").await;
        response.assert_status_ok();
        response.assert_text("Hello, GET");
    }

    #[tokio::test]
    async fn test_multiple_nest_routers() {
        async fn route(method: Method) -> String {
            format!("Hello, {}", method)
        }

        let first = Router::new().route("/first", get(route));
        let second = Router::new().route("/second", get(route));

        let app = Router::new()
            .nest("/api/users", first)
            .nest("/api/posts", second);

        let server = TestServer::new(app).unwrap();
        let response = server.get("/api/users/first").await;
        response.assert_status_ok();
        response.assert_text("Hello, GET");

        let response = server.get("/api/posts/second").await;
        response.assert_status_ok();
        response.assert_text("Hello, GET");
    }

    #[tokio::test]
    async fn test_fallback() {
        async fn route(method: Method) -> String {
            format!("Hello, {}", method)
        }

        let first = Router::new().route("/first", get(route));
        let second = Router::new().route("/second", get(route));

        async fn fallback(request: Request) -> (StatusCode, String) {
            (
                StatusCode::NOT_FOUND,
                format!("No route for {}", request.uri().path()),
            )
        }

        let app = Router::new()
            .nest("/api/users", first)
            .nest("/api/posts", second)
            .fallback(fallback);

        let server = TestServer::new(app).unwrap();
        let response = server.get("/wrong").await;
        response.assert_status(StatusCode::NOT_FOUND);
        response.assert_text("No route for /wrong");
    }

    #[tokio::test]
    async fn test_not_allowed_fallback() {
        async fn route(method: Method) -> String {
            format!("Hello, {}", method)
        }

        let first = Router::new().route("/first", get(route));
        let second = Router::new().route("/second", get(route));

        async fn not_allowed(request: Request) -> (StatusCode, String) {
            (
                StatusCode::METHOD_NOT_ALLOWED,
                format!("No route for {}", request.uri().path()),
            )
        }

        let app = Router::new()
            .nest("/api/users", first)
            .nest("/api/posts", second)
            .method_not_allowed_fallback(not_allowed);

        let server = TestServer::new(app).unwrap();
        let response = server.post("/api/users/first").await;
        response.assert_status(StatusCode::METHOD_NOT_ALLOWED);
        response.assert_text("No route for /api/users/first");
    }
}

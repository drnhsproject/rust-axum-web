use axum::{extract::Request, middleware::Next, response::Response};

async fn log_middleware(request: Request, next: Next) -> Response {
        println!("Receive request: {} {} ", request.method(), request.uri());
        let response = next.run(request).await;
        println!("Response generated, status: {}", response.status());
        response
}

async fn request_id_middleware<T>(mut request: Request<T>) -> Request<T> {
    let request_id = "random-id-12345";
    request
        .headers_mut()
        .insert("X-Request-ID", request_id.parse().unwrap());
    request
}

#[cfg(test)]
mod tests {
    use axum::{Router, middleware::{map_request, from_fn}, routing::get};
    use axum_test::TestServer;
    use http::{HeaderMap, Method};

    use super::*;

    #[tokio::test]
    async fn test_log_middleware() {
        async fn route(method: Method, headers: HeaderMap) -> String {
            let request_id = headers.get("X-Request-ID").unwrap();
            format!("Hello, {} - {} ", method, request_id.to_str().unwrap())
        }

        let app = Router::new()
            .route_service("/get", get(route))
            .layer(map_request(request_id_middleware))
            .layer(from_fn(log_middleware));

        let server = TestServer::new(app).unwrap();
        let response = server.get("/get").await;
        response.assert_status_ok();
        response.assert_text("Hello, GET - random-id-12345 ");
    }
}
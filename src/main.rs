mod login_request;
use axum::{Router, routing::get, serve};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

    let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
    serve(listener, app).await.unwrap();
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use axum::{extract::{Path, Query, Request}, routing::post};
    use axum_test::TestServer;
    use http::{HeaderMap, Method, Uri};

    #[tokio::test]
    async fn test_axum() {
        let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }));

        let server = TestServer::new(app).unwrap();
        let response = server.get("/").await;

        response.assert_status_ok();
        response.assert_text("Hello, World!");
    }

    #[tokio::test]
    async fn test_method_routing() {
        async fn hello_world() -> String {
            "Hello, World!".to_string()
        }

        let app = Router::new()
            .route("/get", get(hello_world))
            .route("/post", post(hello_world));

        let server = TestServer::new(app).unwrap();
        let response = server.get("/get").await;
        response.assert_status_ok();
        response.assert_text("Hello, World!");

        let response = server.post("/post").await;
        response.assert_status_ok();
        response.assert_text("Hello, World!");
    }

    #[tokio::test]
    async fn test_request() {
        async fn hello_world() -> String {
            "Hello, World!".to_string()
        }

        let app = Router::new()
            .route("/get", get(hello_world))
            .route("/post", post(hello_world));

        let server = TestServer::new(app).unwrap();
        let response = server.get("/get").await;
        response.assert_status_ok();
        response.assert_text("Hello, World!");

        let response = server.post("/post").await;
        response.assert_status_ok();
        response.assert_text("Hello, World!");
    }

    #[tokio::test]
    async fn test_request_hello_world() {
        async fn hello_world(request: Request) -> String {
            format!("Hello, {}", request.method())
        }

        let app = Router::new()
            .route("/get", get(hello_world))
            .route("/post", post(hello_world));

        let server = TestServer::new(app).unwrap();
        let response = server.get("/get").await;
        response.assert_status_ok();
        response.assert_text("Hello, GET");

        let response = server.post("/post").await;
        response.assert_status_ok();
        response.assert_text("Hello, POST");
    }

    #[tokio::test]
    async fn test_request_extractor() {
        async fn route(uri: Uri, method: Method) -> String {
            format!("Hello, {} {}", method.as_str(), uri.path())
        }

        let app = Router::new()
            .route("/uri", get(route));

        let server = TestServer::new(app).unwrap();
        let response = server.get("/uri").await;
        response.assert_status_ok();
        response.assert_text("Hello, GET /uri");
    }

    #[tokio::test]
    async fn test_request_extractor_query() {
        async fn route(Query(params): Query<HashMap<String, String>>) -> String {
            let name = params.get("name").unwrap();
            format!("Hello, {}", name)
        }

        let app = Router::new()
            .route("/uri", get(route));

        let server = TestServer::new(app).unwrap();
        let response = server.get("/uri").add_query_param("name", "World").await;
        response.assert_status_ok();
        response.assert_text("Hello, World");
    }

    #[tokio::test]
    async fn test_request_extractor_header() {
        async fn route(headers: HeaderMap) -> String {
            let name = headers["name"].to_str().unwrap();
            format!("Hello, {}", name)
        }

        let app = Router::new()
            .route("/uri", get(route));

        let server = TestServer::new(app).unwrap();
        let response = server.get("/uri").add_header("name", "World").await;
        response.assert_status_ok();
        response.assert_text("Hello, World");
    }

    #[tokio::test]
    async fn test_path_parameter() {
        async fn route(Path((id, id_category)): Path<(String, String)>) -> String {
            format!("Product id {}, Category {}", id, id_category)
        }

        let app = Router::new()
            .route("/products/{id}/category/{id_category}", get(route));

        let server = TestServer::new(app).unwrap();
        let response = server.get("/products/123/category/12").await;
        response.assert_status_ok();
        response.assert_text("Product id 123, Category 12");
    }
}

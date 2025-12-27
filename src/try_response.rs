#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct AuthResponse {
    pub token: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Json, Router, body::Body, extract::Request, response::Response};
    use axum_test::TestServer;
    use http::StatusCode;

    #[tokio::test]
    async fn test_try_response() {
        async fn route(request: Request) -> Response {
            Response::builder()
                .status(StatusCode::OK)
                .header("X-Owner", "hadi")
                .body(Body::from(format!("Hello {}", request.method())))
                .unwrap()
        }

        let app = Router::new()
            .route("/", axum::routing::get(route));

        let server = TestServer::new(app).unwrap();
        let response = server.get("/").await;
        response.assert_status_ok();
        response.assert_header("X-Owner", "hadi");
        response.assert_text("Hello GET");
    }

    #[tokio::test]
    async fn test_response_json() {
        async fn route() -> Json<AuthResponse> {
            Json(AuthResponse {
                token: "TOKEN".to_string(),
            })
        }

        let app = Router::new()
            .route("/get", axum::routing::get(route));

        let server = TestServer::new(app).unwrap();
        let response = server.get("/get").await;
        response.assert_status_ok();
        response.assert_text_contains("\"token\":\"TOKEN\"");
    }

    #[tokio::test]
    async fn test_tuple_into_response() {
        async fn route() -> (Response<()>, Json<AuthResponse>) {
            let resp = Response::builder()
                .status(StatusCode::OK)
                .header("X-Owner", "hadi")
                .body(())
                .unwrap();

            let json = Json(AuthResponse {
                token: "TOKEN".to_string(),
            });

            (resp, json)
        }

        let app = Router::new()
            .route("/create", axum::routing::post(route));

        let server = TestServer::new(app).unwrap();
        let response = server.post("/create").await;

        response.assert_status(StatusCode::OK);
        response.assert_header("X-Owner", "hadi");
        response.assert_text_contains("\"token\":\"TOKEN\"");

        println!("Response: {:?}", response);
    }
}
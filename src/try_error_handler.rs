use axum::response::{IntoResponse, Response};
use http::{StatusCode};

struct DomainException{
    code: i32,
    message: String,
}

impl IntoResponse for DomainException {
    fn into_response(self) -> Response {
       (
        StatusCode::from_u16(self.code as u16).unwrap(),
        self.message
       ).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Router, body::Body, error_handling::HandleError, extract::Request, response::Response, routing::{get, post}};
    use axum_test::TestServer;
    use http::{Method};

    #[tokio::test]
    async fn test_domain_exception() {
        async fn route(method: Method) -> Result<String, DomainException> {
            match method {
                Method::POST => Ok("Hello, World!".to_string()),
                _ => Err(DomainException {
                    code: 405,
                    message: "Method Not Allowed".to_string(),
                }),
            }
        }

        let app = Router::new()
            .route_service("/get", get(route))
            .route("/post", post(route));
        
        let server = TestServer::new(app).unwrap();   
        let response = server.get("/get").await;
        
        response.assert_status(StatusCode::METHOD_NOT_ALLOWED);
        response.assert_text("Method Not Allowed");
    }


    #[tokio::test]
    async fn test_unexpected_error() {
        async fn route(request: Request) -> Result<Response, anyhow::Error> {
            match request.method() {
                &Method::POST => Ok(Response::new(Body::empty())),
                _ => Err(
                    anyhow::anyhow!("Unexpected error occurred")
                ),
            }
        }

        let route_service = tower::service_fn(route);

        async fn handler_error(err: anyhow::Error) -> (StatusCode, String) {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Internal Server Error: {}", err)
            )
        }

        let app = Router::new()
            .route_service("/get", HandleError::new(route_service, handler_error));

        let server = TestServer::new(app).unwrap();   
        let response = server.get("/get").await;
        response.assert_status(StatusCode::INTERNAL_SERVER_ERROR);
        response.assert_text("Internal Server Error: Unexpected error occurred");
    }
}
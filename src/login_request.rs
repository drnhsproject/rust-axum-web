use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[cfg(test)]
mod tests {
    use axum::{Json, Router, extract::rejection::JsonRejection, routing::post};
    use axum_test::TestServer;

    use super::*;

    #[tokio::test]
    async fn test_body_json() {
        async fn route(Json(request): Json<LoginRequest>) -> String {
            format!(
                "Username: {}, Password: {}",
                request.username, request.password
            )
        }
        let app = Router::new().route("/login", post(route));

        let request = LoginRequest {
            username: "test_user".to_string(),
            password: "test_password".to_string(),
        };

        let server = TestServer::new(app).unwrap();
        let response = server
            .post("/login")
            .json(&request)
            .await;
        response.assert_status_ok();
        response.assert_text("Username: test_user, Password: test_password");
    }

    #[tokio::test]
    async fn test_invalid_json() {
        async fn route(payload: Result<Json<LoginRequest>, JsonRejection>) -> String {
           match payload {
                Ok(request) => format!(
                    "Username: {}, Password: {}",
                    request.username, request.password
                ),
                Err(_) => "Invalid JSON".to_string(),
            }
        }

           let app = Router::new().route("/login", post(route));

        let request = LoginRequest {
            username: "test_user".to_string(),
            password: "test_password".to_string(),
        };

        let server = TestServer::new(app).unwrap();
        let response = server
            .post("/login")
            .json(&request)
            .await;
        response.assert_status_ok();
        response.assert_text("Username: test_user, Password: test_password");
    }
}
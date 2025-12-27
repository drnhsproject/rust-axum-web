#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct LoginFormRequest {
    pub username: String,
    pub password: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::{Router, body::Bytes, extract::{Form, Multipart}, routing::post};
    use axum_test::{TestServer, multipart::{MultipartForm, Part}};

    #[tokio::test]
    async fn test_login_form() {
        async fn route(Form(form): Form<LoginFormRequest>) -> String {
            format!("Hello, {}!", form.username)
        }

        let app = Router::new()
            .route("/login", post(route));

        let server = TestServer::new(app).unwrap();
        let response = server.post("/login")
            .form(&LoginFormRequest {
                username: "hadi".to_string(),
                password: "password".to_string(),
            })
            .await;

        response.assert_status_ok();
        response.assert_text("Hello, hadi!");
    }

    #[tokio::test]
    async fn test_multipart_form() {
        async fn route(mut multipart: Multipart) -> String {
            let mut profile: Bytes = Bytes::new();
            let mut username = "".to_string();

            while let Some(field) = multipart.next_field().await.unwrap() {
                if field.name().unwrap_or("") == "profile" {
                    profile = field.bytes().await.unwrap();
                } else if field.name().unwrap_or("") == "username" {
                    username = field.text().await.unwrap();
                }
            }

            assert!(profile.len() > 0);
            format!("Hello, {}!", username)
        }

        let app = Router::new()
            .route("/upload", post(route)); 

        let request = MultipartForm::new()
            .add_text("username", "hadi")
            .add_text("password", "password")
            .add_part("profile", Part::bytes(Bytes::from("profile")));

        let server = TestServer::new(app).unwrap();
        let response = server.post("/upload")
            .multipart(request)
            .await; 

        response.assert_status_ok();
        response.assert_text("Hello, hadi!");
    }
}
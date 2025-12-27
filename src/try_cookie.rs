#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use axum::{Router, extract::Query, routing::get};
    use axum_extra::extract::{CookieJar, cookie::Cookie};
    use axum_test::TestServer;

    #[tokio::test]
    async fn test_cookie_extraction() {
        async fn route(query: Query<HashMap<String, String>>) -> (CookieJar, String) {
            let name = query.get("name").unwrap();
            (
                CookieJar::new().add(Cookie::new("name", name.clone())),    
                format!("Hello, {}!", name)
            )
        }

        let app = Router::new()
            .route("/set_cookie", get(route));
        let server = TestServer::new(app).unwrap();
        let response = server.get("/set_cookie")
            .add_query_param("name", "AxumUser")
            .await;

        response.assert_status_ok();
        response.assert_text("Hello, AxumUser!");
        response.assert_contains_header("Set-Cookie");
        response.assert_header("Set-Cookie", "name=AxumUser");
    }

    #[tokio::test]
    async fn test_cookie_request() {
        async fn route(cookies: CookieJar) -> String {
            format!("Hello, {}!", cookies.get("name").unwrap().value())
        }

        let app = Router::new()
            .route("/get_cookie", get(route));
        let server = TestServer::new(app).unwrap();
        let response = server.get("/get_cookie")
            .add_cookie(Cookie::new("name", "AxumUser"))
            .await;

        response.assert_status_ok();
        response.assert_text("Hello, AxumUser!");
    }
}
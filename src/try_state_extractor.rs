struct DatabaseConfig {
    total: i32
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use axum::{Extension, Router, extract::State, routing::get};
    use axum_test::TestServer;

    use super::*;

    #[tokio::test]
    async fn test_database_config() {
        let database_state = Arc::new(DatabaseConfig{ total: 100 });

        async fn route(State(database): State<Arc<DatabaseConfig>>) -> String {
            format!("Total database connections: {}", database.total)
        }

        let app = Router::new()
            .route("/", get(route))
            .with_state(database_state);
        let server = TestServer::new(app).unwrap();
        let response = server.get("/").await;

        response.assert_status_ok();
        response.assert_text("Total database connections: 100");
    }

    #[tokio::test]
    async fn test_state_extension() {
        let database_state = Arc::new(DatabaseConfig{ total: 100 });

        async fn route(Extension(database): Extension<Arc<DatabaseConfig>>) -> String {
            format!("Total database connections: {}", database.total)
        }

        let app = Router::new()
            .route("/", get(route))
            .layer(Extension(database_state)); // IF Forget this line, test will fail 500

        let server = TestServer::new(app).unwrap();
        let response = server.get("/").await;
        
        response.assert_status_ok();
        response.assert_text("Total database connections: 100");
    }

    #[tokio::test]
    async fn test_closure_capture() {
        let database_state = Arc::new(DatabaseConfig{ total: 100 });

        async fn route(database: Arc<DatabaseConfig>) -> String {
            format!("Total database connections: {}", database.total)
        }

        let app = Router::new()
            .route("/", 
            get({
                let database_state = Arc::clone(&database_state);
                move || route(database_state)
            }));

        let server = TestServer::new(app).unwrap();
        let response = server.get("/").await;
        
        response.assert_status_ok();
        response.assert_text("Total database connections: 100");
    }
}

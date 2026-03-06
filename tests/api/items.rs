use crate::helpers::TestApp;
use axum::http::StatusCode;

#[tokio::test]
async fn should_successfully_create_item() {
    let app = TestApp::new().await;

    let body = serde_json::json!({
         "name":"Wrench",
         "description":"A tool used for gripping and turning objects.",
         "quantity": 2,
         "tags": ["tool", "hardware", "metal" ]
    });

    let response = app.create_item(&body).await;

    assert_eq!(response.status(), StatusCode::CREATED);
}

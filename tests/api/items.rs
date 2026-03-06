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

#[tokio::test]
async fn should_return_422_if_malformed_input() {
    let app = TestApp::new().await;

    let body = serde_json::json!({
         "_name":"Wrench",
         "description":"A tool used for gripping and turning objects.",
         "quantity": 2,
         "tags": ["tool", "hardware", "metal" ]
    });

    let response = app.create_item(&body).await;

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}

#[tokio::test]
async fn should_return_422_if_missing_required_fields() {
    let app = TestApp::new().await;

    let body = serde_json::json!({
         "description":"A tool used for gripping and turning objects.",
         "quantity": 2,
         "tags": ["tool", "hardware", "metal" ]
    });

    let response = app.create_item(&body).await;

    assert_eq!(response.status(), StatusCode::UNPROCESSABLE_ENTITY);
}



#[tokio::test]
async fn should_list_items() {
    let app = TestApp::new().await;

    let body = serde_json::json!({
        "name":"Hammer",
        "description":"Tool",
        "quantity": 5,
        "tags": ["tool"]
    });

    app.create_item(&body).await;

    let response = app.list_items().await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn should_get_item_by_id() {
    let app = TestApp::new().await;

    let body = serde_json::json!({
        "name":"Hammer",
        "description":"Tool",
        "quantity": 5,
        "tags": ["tool"]
    });

    let response = app.create_item(&body).await;
    assert_eq!(response.status(), StatusCode::CREATED);

    let get_response = app.get_item(1).await;

    assert_eq!(get_response.status(), StatusCode::OK);
}

#[tokio::test]
async fn should_return_404_when_getting_nonexistent_item() {
    let app = TestApp::new().await;

    let response = app.get_item(999).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn should_successfully_update_item() {
    let app = TestApp::new().await;

    let body = serde_json::json!({
        "name":"Hammer",
        "description":"Tool",
        "quantity": 5,
        "tags": ["tool"]
    });

    app.create_item(&body).await;

    let update = serde_json::json!({
        "name":"Hammer Updated",
        "description":"Updated description",
        "quantity": 10,
        "tags": ["tool", "updated"]
    });

    let response = app.update_item(1, &update).await;

    assert_eq!(response.status(), StatusCode::OK);
}

#[tokio::test]
async fn should_return_404_when_updating_nonexistent_item() {
    let app = TestApp::new().await;

    let update = serde_json::json!({
        "name":"Hammer Updated",
        "description":"Updated description",
        "quantity": 10,
        "tags": ["tool"]
    });

    let response = app.update_item(999, &update).await;

    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn should_delete_item() {
    let app = TestApp::new().await;

    let body = serde_json::json!({
        "name":"Hammer",
        "description":"Tool",
        "quantity": 5,
        "tags": ["tool"]
    });

    app.create_item(&body).await;

    let response = app.delete_item(1).await;

    assert_eq!(response.status(), StatusCode::NO_CONTENT);
}

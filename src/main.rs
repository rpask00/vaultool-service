use std::sync::Arc;
use tokio::sync::RwLock;
use vaultility_service::app_state::AppState;
use vaultility_service::services::data_stores::items::PostgresItemsStore;
use vaultility_service::utils::constant::prod;
use vaultility_service::{Application, get_postgres_pool};

#[tokio::main]
async fn main() {
    color_eyre::install().expect("Failed to install color_eyre");

    let poll = get_postgres_pool(
        std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set!")
            .into(),
    )
    .await
    .expect("Failed to create Postgres poll");

    let items_store = Arc::new(RwLock::new(PostgresItemsStore::new(poll)));

    let app_state = AppState::new(items_store);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to start app!");
}

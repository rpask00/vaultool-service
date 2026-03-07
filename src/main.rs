use std::sync::Arc;
use tokio::sync::RwLock;
use vaultool_service::app_state::AppState;
use vaultool_service::services::data_stores::{postgres_files_store::PostgresFilesStore, postgres_items_store::PostgresItemsStore};
use vaultool_service::utils::constant::prod;
use vaultool_service::{Application, get_postgres_pool};
use vaultool_service::utils::tracing::init_tracing;

#[tokio::main]
async fn main() {
    init_tracing().expect("Failed to initialize tracing");
    color_eyre::install().expect("Failed to install color_eyre");

    let pool = get_postgres_pool(
        std::env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set!")
            .into(),
    )
    .await
    .expect("Failed to create Postgres poll");

    let items_store = Arc::new(RwLock::new(PostgresItemsStore::new(pool.clone())));
    let files_store = Arc::new(RwLock::new(PostgresFilesStore::new(pool)));

    let app_state = AppState::new(items_store, files_store);

    let app = Application::build(app_state, prod::APP_ADDRESS)
        .await
        .expect("Failed to build app");

    app.run().await.expect("Failed to start app!");
}

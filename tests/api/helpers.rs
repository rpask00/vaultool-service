use dotenv::dotenv;
use reqwest::cookie::Jar;
use sqlx::postgres::{PgConnectOptions, PgPoolOptions};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use std::str::FromStr;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;
use vaultool_service::app_state::AppState;
use vaultool_service::services::data_stores::items::PostgresItemsStore;
use vaultool_service::utils::constant::test;
use vaultool_service::{get_postgres_pool, Application};

pub struct TestApp {
    pub address: String,
    pub http_client: reqwest::Client,
    pub cookie_jar: Arc<Jar>,
    #[allow(dead_code)]
    pub db_name: String,
}

impl TestApp {
    pub async fn new() -> Self {
        let (pg_pool, db_name) = configure_postgresql().await;

        let items_store = Arc::new(RwLock::new(PostgresItemsStore::new(pg_pool)));

        let app_state = AppState::new(items_store);

        let cookie_jar = Arc::new(Jar::default());

        let app = Application::build(app_state, test::APP_ADDRESS)
            .await
            .expect("Failed to build app");

        let address = format!("http://{}", app.address.clone());

        let _ = tokio::spawn(app.run());

        let http_client = reqwest::Client::builder()
            .cookie_provider(Arc::clone(&cookie_jar))
            .build()
            .unwrap();

        TestApp {
            address,
            http_client,
            cookie_jar,
            db_name,
        }
    }

    #[allow(dead_code)]
    pub async fn get_root(&self) -> reqwest::Response {
        self.http_client
            .get(&format!("{}/", &self.address))
            .send()
            .await
            .expect("Failed to execute request.")
    }

    pub async fn create_item<Body>(&self, body: &Body) -> reqwest::Response
    where
        Body: serde::Serialize,
    {
        self.http_client
            .post(format!("{}/items", &self.address))
            .json(body)
            .send()
            .await
            .expect("Failed to execute request.")
    }

}


async fn configure_postgresql() -> (PgPool, String) {
    dotenv().ok();

    // We are creating a new database for each test case, and we need to ensure each database has a unique name!
    let db_name = format!("{}{}", test::TEST_DB_PREFIX, Uuid::new_v4().to_string());

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set")
        .to_owned()
        .replace("/vaultool", "");

    configure_database(&database_url, &db_name).await;

    let postgresql_conn_url_with_db = format!("{}/{}", database_url, db_name);

    // Create a new connection pool and return it
    (
        get_postgres_pool(postgresql_conn_url_with_db.into())
            .await
            .expect("Failed to create Postgres connection pool!"),
        db_name,
    )
}

async fn configure_database(db_conn_string: &str, db_name: &str) {
    // Create database connection
    let connection = PgPoolOptions::new()
        .connect(db_conn_string)
        .await
        .expect("Failed to create Postgres connection pool.");

    // Create a new database
    connection
        .execute(format!(r#"CREATE DATABASE "{}";"#, db_name).as_str())
        .await
        .expect("Failed to create database.");

    // Connect to new database
    let db_conn_string = format!("{}/{}", db_conn_string, db_name);

    let connection = PgPoolOptions::new()
        .connect(&db_conn_string)
        .await
        .expect("Failed to create Postgres connection pool.");

    // Run migrations against new database
    sqlx::migrate!()
        .run(&connection)
        .await
        .expect("Failed to migrate the database");
}


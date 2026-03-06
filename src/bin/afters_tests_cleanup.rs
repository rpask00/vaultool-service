// use dotenv::dotenv;
// use sqlx::postgres::PgConnectOptions;
// use sqlx::{Connection, Executor, PgConnection};
// use std::str::FromStr;
// use vaultool_service::utils::constant::test;
//
// #[tokio::main]
// async fn main() {
//     dotenv().ok();
//
//     let database_name = std::env::var("DATABASE_NAME")
//         .expect("DATABASE_NAME must be set")
//         .to_owned();
//
//     let database_url = std::env::var("DATABASE_URL")
//         .expect("DATABASE_URL must be set")
//         .to_owned()
//         .replace(format!("/{}", database_name).as_str(), "");
//
//     println!("{}", database_url);
//     let connection_options = PgConnectOptions::from_str(&database_url)
//         .expect("Failed to parse PostgreSQL connection string");
//
//     let mut connection = PgConnection::connect_with(&connection_options)
//         .await
//         .expect("Failed to connect to Postgres");
//
//     connection
//         .execute(
//             format!(
//                 r#"
//                 SELECT 'DROP DATABASE "' || datname || '";'
//                 FROM pg_database
//                 WHERE datname LIKE '{}%';
//         "#,
//                 test::TEST_DB_PREFIX
//             )
//             .as_str(),
//         )
//         .await
//         .expect("Failed to drop databases.");
// }

use dotenv::dotenv;
use sqlx::postgres::PgConnectOptions;
use sqlx::{Connection, Executor, PgConnection, Row};
use std::str::FromStr;
use vaultool_service::utils::constant::test;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_name =
        std::env::var("DATABASE_NAME").expect("DATABASE_NAME must be set");

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set")
        .replace(&format!("/{}", database_name), "");

    let connection_options =
        PgConnectOptions::from_str(&database_url).expect("Failed to parse connection string");

    let mut connection = PgConnection::connect_with(&connection_options)
        .await
        .expect("Failed to connect");

    let rows = sqlx::query(
        r#"
        SELECT datname
        FROM pg_database
        WHERE datname LIKE $1
        "#,
    )
        .bind(format!("{}%", test::TEST_DB_PREFIX))
        .fetch_all(&mut connection)
        .await
        .expect("Failed to fetch databases");

    for row in rows {
        let db_name: String = row.get("datname");

        let drop_query = format!(r#"DROP DATABASE "{}""#, db_name);
        println!("Dropping {}", db_name);

        connection
            .execute(drop_query.as_str())
            .await
            .expect("Failed to drop database");
    }
}
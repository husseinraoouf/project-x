pub mod models;
pub mod schemas;
pub mod utils;

use log::error;
pub use schemas::postgresql::schema;

use std::process::exit;

use diesel::{
    r2d2::{ConnectionManager, Pool, PooledConnection},
    PgConnection,
};

use tokio::time::Duration;

use crate::config::CONFIG;

pub use utils::blocking_database;

#[derive(Clone)]
pub struct DbPool {
    // This is an 'Option' so that we can drop the pool in a 'spawn_blocking'.
    pool: Pool<ConnectionManager<PgConnection>>,
}

impl DbPool {
    pub async fn from_config() -> DbPool {
        match utils::retry_db_connection(|| DbPool::internal_from_config(), 10).await {
            Ok(p) => p,
            Err(e) => {
                error!("Error creating database pool: {:?}", e);
                exit(1);
            }
        }
    }

    fn internal_from_config() -> Result<Self, std::io::Error> {
        let url = CONFIG.get_database_url();

        let manager = ConnectionManager::new(&url);
        let pool = Pool::builder()
            .max_size(10)
            .connection_timeout(Duration::from_secs(10))
            .build(manager)
            .unwrap_or_else(move |_| panic!("Error connecting to {}", &url));

        Ok(DbPool { pool })
    }

    // Get a connection from the pool
    pub async fn get(&self) -> PooledConnection<ConnectionManager<PgConnection>> {
        let duration = Duration::from_secs(10);

        let pool = self.pool.clone();

        let connection = actix_web::web::block(move || pool.get_timeout(duration).unwrap())
            .await
            .unwrap();

        connection
    }
}

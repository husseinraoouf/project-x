use log::*;

use tokio::time::{sleep, Duration};

use super::DbPool;

pub async fn retry_db_connection<F, T, E>(func: F, max_tries: u32) -> Result<T, E>
where
    F: Fn() -> Result<T, E>,
    E: std::error::Error,
{
    let mut tries = 0;

    loop {
        match func() {
            ok @ Ok(_) => return ok,
            Err(e) => {
                tries += 1;

                if tries >= max_tries && max_tries > 0 {
                    return Err(e);
                }

                warn!("Can't connect to database, retrying: {:?}", e);

                sleep(Duration::from_millis(1_000)).await;
            }
        }
    }
}

embed_migrations!("migrations/postgresql");

pub async fn run_migrations(pool: &DbPool) -> Result<(), std::io::Error> {
    use diesel::RunQueryDsl;

    // Make sure the database is up to date (create if it doesn't exist, or run the migrations)
    let connection = pool.get().await;
    // Disable Foreign Key Checks during migration

    // FIXME: Per https://www.postgresql.org/docs/12/sql-set-constraints.html,
    // "SET CONSTRAINTS sets the behavior of constraint checking within the
    // current transaction", so this setting probably won't take effect for
    // any of the migrations since it's being run outside of a transaction.
    // Migrations that need to disable foreign key checks should run this
    // from within the migration script itself.
    diesel::sql_query("SET CONSTRAINTS ALL DEFERRED")
        .execute(&connection)
        .expect("Failed to disable Foreign Key Checks during migrations");

    embedded_migrations::run_with_output(&connection, &mut std::io::stdout()).unwrap();
    Ok(())
}

// A wrapper around actix_web::web::block that propagates panics to the calling code.
pub async fn blocking_database<F, T>(pool: &DbPool, f: F) -> T
where
    F: FnOnce(&diesel::PgConnection) -> T + Send + 'static,
    T: Send + 'static,
{
    let pool = pool.clone();
    let conn = pool.get().await;
    let res = actix_web::web::block(move || (f)(&conn)).await.unwrap();

    res
}

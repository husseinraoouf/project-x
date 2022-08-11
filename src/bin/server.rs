extern crate projectx;

use projectx::{
    api::lunch_server,
    db::{utils::run_migrations, DbPool},
    logging::init_logger,
};

#[actix_web::main]
async fn main() {
    // Configure logger
    init_logger();

    // create db connections pool
    let pool = DbPool::from_config().await;

    // run database migrations
    run_migrations(&pool).await.unwrap();

    // launch actix server
    lunch_server(pool).await
}

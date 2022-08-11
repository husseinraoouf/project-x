pub mod context;
pub mod graphql;
pub mod identity;

use actix_web::{web::Data, App, HttpServer};

use crate::{api::context::Context, config::CONFIG, db::DbPool};

use self::{
    graphql::create_graphql_schema, graphql::routes as graphql_routes,
    identity::routes as identity_routes,
};

pub async fn lunch_server(pool: DbPool) {
    // create graphql schema
    // we need to create graphql schema here instead of in the graphql_route to be created only once
    let schema = create_graphql_schema();

    log::info!("Starting http server at {}:{}", CONFIG.bind, CONFIG.port);

    HttpServer::new(move || {
        let context = Context::create(pool.clone(), CONFIG.to_owned());

        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(Data::new(context))
            .service(identity_routes())
            .service(graphql_routes(schema.clone()))
    })
    .bind((CONFIG.bind, CONFIG.port))
    .unwrap()
    .run()
    .await
    .unwrap();
}

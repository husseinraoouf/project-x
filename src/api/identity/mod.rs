use actix_web::{
    get, post,
    web::{self},
};

use super::context::Context;

pub fn routes() -> actix_web::Scope {
    web::scope("/identity") // Set schema in app data
        // playground
        .service(index_playground)
        // graphql endpoint
        .service(index)
}

#[post("/hello")]
async fn index(_context: web::Data<Context>) -> &'static str {
    "Hello"
}

#[get("/hello")]
async fn index_playground(_context: web::Data<Context>) -> &'static str {
    "Hello"
}

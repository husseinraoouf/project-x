use actix_web::{get, guard, web, web::Data, App, HttpResponse, HttpServer, Result};
use async_graphql::{
    http::{playground_source, GraphQLPlaygroundConfig},
    EmptyMutation, EmptySubscription, Object, Schema,
};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

type StarWarsSchema = Schema<Query, EmptyMutation, EmptySubscription>;

struct Query;

#[Object]
impl Query {
    /// Returns Hello, World
    async fn hello(&self) -> &str {
        "Hello, World"
    }
}

async fn index(schema: web::Data<StarWarsSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[get("/")]
async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let schema = Schema::build(Query, EmptyMutation, EmptySubscription).finish();

    println!("Playground: http://localhost:8000");

    HttpServer::new(move || {
        App::new()
            // Set schema in app data
            .app_data(Data::new(schema.clone()))
            // playground
            .service(index_playground)
            // graphql endpoint
            .service(web::resource("/").guard(guard::Post()).to(index))
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
}

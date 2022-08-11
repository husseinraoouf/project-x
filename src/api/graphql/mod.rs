use actix_web::{
    get, post,
    web::{self, Data},
    HttpResponse, Result,
};
use async_graphql::http::{playground_source, GraphQLPlaygroundConfig};
use async_graphql_actix_web::{GraphQLRequest, GraphQLResponse};

use async_graphql::{extensions, EmptySubscription, MergedObject, Schema};

use self::modules::{
    devtools::{DevtoolsModuleMutation, DevtoolsModuleQuery},
    user::{UserModuleMutation, UserModuleQuery},
};

pub mod modules;

#[derive(MergedObject, Default)]
pub struct Query(UserModuleQuery, DevtoolsModuleQuery);

#[derive(MergedObject, Default)]
pub struct Mutation(UserModuleMutation, DevtoolsModuleMutation);

pub type GraphqlSchema = Schema<Query, Mutation, EmptySubscription>;

pub fn create_graphql_schema() -> GraphqlSchema {
    Schema::build(Query::default(), Mutation::default(), EmptySubscription)
        .extension(extensions::Logger)
        .finish()
}

pub fn routes(schema: GraphqlSchema) -> actix_web::Scope {
    web::scope("/graphql") // Set schema in app data
        .app_data(Data::new(schema))
        // playground
        .service(index_playground)
        // graphql endpoint
        .service(index)
}

#[post("")]
async fn index(schema: web::Data<GraphqlSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

#[get("")]
async fn index_playground() -> Result<HttpResponse> {
    let source = playground_source(GraphQLPlaygroundConfig::new("/").subscription_endpoint("/"));
    Ok(HttpResponse::Ok()
        .content_type("text/html; charset=utf-8")
        .body(source))
}

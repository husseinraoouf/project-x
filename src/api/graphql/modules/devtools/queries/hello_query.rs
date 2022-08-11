use async_graphql::Object;

#[derive(Default)]
pub struct HelloQuery;

#[Object]
impl HelloQuery {
    async fn hello(&self) -> &str {
        "Hello"
    }
}

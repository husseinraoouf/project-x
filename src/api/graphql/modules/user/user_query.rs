use async_graphql::Object;

#[derive(Default)]
pub struct UserQuery;

#[Object]
impl UserQuery {
    async fn user(&self) -> &str {
        "Hello"
    }
}

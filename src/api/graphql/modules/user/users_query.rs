use async_graphql::Object;

#[derive(Default)]
pub struct UsersQuery;

#[Object]
impl UsersQuery {
    async fn users(&self) -> &str {
        "Hello"
    }
}

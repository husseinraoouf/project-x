use async_graphql::Object;

#[derive(Default)]
pub struct HelloMutation;

#[Object]
impl HelloMutation {
    async fn hello(&self) -> &str {
        "Hello"
    }
}

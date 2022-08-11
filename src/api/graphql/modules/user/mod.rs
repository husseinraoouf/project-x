pub mod dataloaders;
pub mod user_query;
pub mod users_query;

use async_graphql::MergedObject;

use self::{user_query::UserQuery, users_query::UsersQuery};

#[derive(MergedObject, Default)]
pub struct UserModuleQuery(UserQuery, UsersQuery);

#[derive(MergedObject, Default)]
pub struct UserModuleMutation();

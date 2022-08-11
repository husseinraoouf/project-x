pub mod hello_query;

use async_graphql::MergedObject;

use self::hello_query::HelloQuery;

#[derive(MergedObject, Default)]
pub struct DevtoolsModuleQuery(HelloQuery);

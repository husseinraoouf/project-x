pub mod hello_mutation;

use async_graphql::MergedObject;

use self::hello_mutation::HelloMutation;

#[derive(MergedObject, Default)]
pub struct DevtoolsModuleMutation(HelloMutation);

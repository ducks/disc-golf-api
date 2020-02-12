use juniper::{Context as JuniperContext, FieldResult, RootNode};

use crate::graphql::models::user::{User, UserCreate};
use crate::db::Postgres;

pub struct Context {
    pub connection: Postgres
}

impl JuniperContext for Context {}

pub struct QueryRoot;

graphql_object!(QueryRoot: Context |&self| {
  field user(&executor) -> FieldResult<User> {
    let user = User {
      created: 1551909311,
      account_number: 123456789,
    };
    Ok(user)
  }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: Context |&self| {
  field add_user(&executor, user_create: UserCreate) -> FieldResult<User> {
    let user = User {
      created: user_create.created,
      account_number: user_create.account_number,
    };
    Ok(user)
  }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;
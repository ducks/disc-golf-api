use juniper::{Context as JuniperContext, FieldResult, RootNode};

use crate::graphql::models::user::{User};
use crate::db::Postgres;

use uuid::Uuid;

pub struct Context {
    pub connection: Postgres
}

impl JuniperContext for Context {}

pub struct QueryRoot;

graphql_object!(QueryRoot: Context |&self| {
    field user(&executor) -> FieldResult<User> {
        let user = User {
            created: 1551909311,
            account_number: "123456789",
        };
        Ok(user)
    }
});

pub struct MutationRoot;

graphql_object!(MutationRoot: Context |&self| {
    field add_user(&executor) -> FieldResult<User> {
        let account_number = Uuid::new_v4().hyphenated().to_string();
        let created = 20200229;

        let user = User {
            created: created,
            account_number: &account_number,
        };
        Ok(user)
    }
});

pub type Schema = RootNode<'static, QueryRoot, MutationRoot>;

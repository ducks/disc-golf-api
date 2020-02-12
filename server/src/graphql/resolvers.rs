use rocket::response::content::Html;
use rocket::State;
use juniper_rocket::{
    graphiql_source, 
    GraphQLRequest, 
    GraphQLResponse
};

use crate::db::Postgres;
use crate::graphql::schema::{Context, Schema};

#[get("/")]
pub fn graphiql() -> Html<String> {
  graphiql_source("/graphql")
}

#[get("/graphql?<request>")]
pub fn get_graphql_handler(
  context: Postgres,
  request: GraphQLRequest,
  schema: State<Schema>,
) -> GraphQLResponse {
  request.execute(&schema, &Context { connection: context })
}

#[post("/graphql", data = "<request>")]
pub fn post_graphql_handler(
  context: Postgres,
  request: GraphQLRequest,
  schema: State<Schema>,
) -> GraphQLResponse {
  request.execute(&schema, &Context { connection: context })
}

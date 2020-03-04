#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
#[macro_use] extern crate juniper;
#[macro_use] extern crate diesel;
extern crate juniper_rocket;

mod db;
mod graphql;
mod schema;

fn main() {
    rocket::ignite()
        .attach(db::Postgres::fairing())
        .manage(graphql::schema::Schema::new(
                    graphql::schema::QueryRoot,
                    graphql::schema::MutationRoot,
                ))
        .mount("/", routes![
               graphql::resolvers::graphiql,
               graphql::resolvers::get_graphql_handler,
               graphql::resolvers::post_graphql_handler
        ])
        .launch();
}

use rocket_contrib::database;

#[database("postgres")]
pub struct Postgres(pub diesel::PgConnection);

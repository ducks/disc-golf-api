pub mod user {
    #[derive(GraphQLObject)]
    pub struct User {
      pub created: i32,
      pub account_number: i32,
    }

    #[derive(GraphQLInputObject)]
    pub struct UserCreate {
      pub created: i32,
      pub account_number: i32,
    }
}

pub mod course {
    #[derive(GraphQLObject)]
    pub struct Course {
        pub id: String,
        pub name: String,
        pub city: String,
        pub state: String,
        pub holes: Option<Vec<Option<crate::graphql::models::hole::Hole>>>,
        pub par: i32,
    }
}

pub mod hole {
    #[derive(GraphQLObject)]
    pub struct Hole {
      pub id: String,
      pub distance: i32,
      pub course_id: String,
      pub par: i32,
    }
}

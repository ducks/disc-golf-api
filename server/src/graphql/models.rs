pub mod user {
    #[derive(GraphQLObject)]
    pub struct User {
      pub created: i32,
      pub account_number: uuid,
    }

    #[derive(GraphQLInputObject)]
    pub struct UserCreate {
      pub created: i32,
      pub account_number: uuid,
    }
}

pub mod course {
    #[derive(GraphQLObject)]
    pub struct Course {
        pub id: uuid,
        pub name: String,
        pub city: String,
        pub state: String,
        pub holes: Option<Vec<Option<Episode>>>,
        pub par: i32,
    }
}

pub mod hole {
    #[derive{GraphQLObject)]
    pub struct Hole {
      pub id: uuid,
      pub distance: i32,
      pub course_id: uuid,
      pub par: i32,
    }
}

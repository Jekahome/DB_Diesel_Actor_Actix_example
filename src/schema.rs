 
 
use diesel::sql_types::*;
use custom_sql_types::Id;

// Custom types
pub mod custom_sql_types {
    #[derive(diesel::sql_types::SqlType)]
    #[diesel(postgres_type(name = "Id"))]
    pub struct Id(i32);

   impl diesel::query_builder::QueryId for Id{
    type QueryId = i32;
    const HAS_STATIC_QUERY_ID: bool = true;
     
    /*fn query_id() -> Option<std::any::TypeId> {
        if Self::HAS_STATIC_QUERY_ID {
            Some(std::any::TypeId::of::<Self::QueryId>())
        } else {
            None
        }
    }*/
   }
   impl diesel::sql_types::SqlOrd for Id {}
}

table! {
    use super::*;
    cars (id) {
        id -> Id,
        id_user -> Id,
        car -> Text,
        description -> Nullable<Text>,
        create_at -> Nullable<Timestamp>,
    }
}

table! {
    use super::*;
    posts (id) {
        id -> Id,
        id_user -> Id,
        title -> Text,
        create_at -> Nullable<Timestamp>,
    }
}

table! {
    use super::*;
    sports (id) {
        id -> Id,
        name -> Varchar,
    }
}

table! {
    use super::*;
    user_sports (id_user, id_sport) {
        id_user -> Id,
        id_sport -> Id,
    }
}

table! {
    use super::*;
    users (id) {
        id -> Id,
        name -> Varchar,
        email -> Varchar,
        age -> Int4,
        obj -> Nullable<Json>,
        create_at -> Nullable<Timestamp>,
    }
}

joinable!(cars -> users (id_user));
joinable!(posts -> users (id_user));
joinable!(user_sports -> sports (id_sport));
joinable!(user_sports -> users (id_user));

allow_tables_to_appear_in_same_query!(
    cars,
    posts,
    sports,
    user_sports,
    users,
);

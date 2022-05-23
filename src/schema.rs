 use diesel::sql_types::*;

table! {
    use super::*;
    cars (id) {
        id -> Int4,
        id_user -> Int4,
        car -> Text,
        description -> Nullable<Text>,
        create_at -> Nullable<Timestamp>,
    }
}

table! {
    use super::*;
    posts (id) {
        id -> Int4,
        id_user -> Int4,
        title -> Text,
        create_at -> Nullable<Timestamp>,
    }
}

table! {
    use super::*;
    sports (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    use super::*;
    user_sports (id_user, id_sport) {
        id_user -> Int4,
        id_sport -> Int4,
    }
}

table! {
    use super::*;
    users (id) {
        id -> Int4,
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

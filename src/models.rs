#![allow(unused_imports)]

use diesel::prelude::*;
use diesel::pg::{Pg, PgValue};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::deserialize::{self, FromSql, FromSqlRow};
// use diesel::pg::data_types::{PgMoney,Cents}; error: Deserialize is not implemented PgMoney
use diesel::sql_types::{Varchar,Integer,Int4}; 
use std::io::Write;
use serde::{Deserialize, Serialize};
//use crate::schema::users::*;
use crate::schema::{users,posts,cars,user_sports,sports}; 
 

// Option<> если поле может быть NULL(в schema Nullable) или имеет DEFAULT
// Для полей допускающих NULL добавить  #[diesel(treat_none_as_null = true)]
 
// model for SELECT
#[derive(Queryable, Identifiable, AsChangeset, QueryableByName, Debug, Clone, Deserialize, Serialize)]
#[diesel(primary_key(id))]
#[diesel(treat_none_as_null = true)]
#[diesel(table_name = users)]
pub struct User {
    #[diesel(sql_type = Int4)]
    pub id: my_type_safety::Id,
    #[diesel(sql_type = Varchar)]
    pub name: String,
    #[diesel(sql_type = Varchar)]
    pub email: String,
    #[diesel(column_name = age)]
    #[diesel(serialize_as = i32)]
    pub age: i32,
    pub obj: Option<serde_json::Value>,// diesel = {features = ["serde_json"]}
    pub create_at: Option<chrono::NaiveDateTime>// chrono = {features = ["serde"] }
}

impl Identifiable for User {
    type Id = my_type_safety::Id;

    fn id(self) -> Self::Id{
        self.id
    }
}

// model for INSERT
#[derive(Debug, Clone, Insertable, AsChangeset, Deserialize, Serialize)]
#[diesel(table_name = users)]
#[diesel(treat_none_as_null = true)]
pub struct NewUser<'a> {
    pub name: &'a str,
    pub email: &'a str,
    pub age: i32,
    pub obj: Option<serde_json::Value>, // diesel = {features = ["serde_json"]} 
    pub create_at: Option<chrono::NaiveDateTime> // diesel = {features = ["chrono"]}
}
 
pub mod my_type_safety{
    use super::*;
    #[derive(Debug, AsExpression, FromSqlRow,Clone,Copy, Deserialize, Serialize, Hash, Eq, PartialEq)]
    #[diesel(sql_type = Int4)]
    pub struct Id(pub i32);

    impl FromSql<Int4, Pg> for Id {
        fn from_sql(bytes: PgValue<'_>) -> deserialize::Result<Self> {
            FromSql::<Integer, Pg>::from_sql(bytes).map(Id)
        }
    }

    impl ToSql<Int4, Pg> for Id {
        fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Pg>) -> serialize::Result {
            ToSql::<Integer, Pg>::to_sql(&self.0, out)
        }
    }
}

#[derive(Queryable, Identifiable, AsChangeset, QueryableByName, Associations,Debug, Clone, Deserialize, Serialize)]
#[diesel(primary_key(id))]
#[diesel(treat_none_as_null = true)]
#[diesel(table_name = posts)]
#[diesel(belongs_to(User, foreign_key = id_user))]
pub struct Post {
    #[diesel(sql_type = Int4)]
    pub id: my_type_safety::Id,
    #[diesel(sql_type = Int4)]
    pub id_user: my_type_safety::Id,
    #[diesel(sql_type = Varchar)]
    pub title: String,
    pub create_at: Option<chrono::NaiveDateTime> // chrono = {features = ["serde"] }
}

#[derive(Queryable,Identifiable,Associations,Debug, Clone, Deserialize, Serialize)]
#[diesel(primary_key(id))]
#[diesel(table_name = cars)]
#[diesel(belongs_to(User, foreign_key = id_user))]
pub struct Car {
    #[diesel(sql_type = Int4)]
    pub id: my_type_safety::Id,
    #[diesel(sql_type = Int4)]
    pub id_user: my_type_safety::Id,
    #[diesel(sql_type = Varchar)]
    pub car: String,
    #[diesel(sql_type = Varchar)]
    pub description: Option<String>,
    pub create_at: Option<chrono::NaiveDateTime>// chrono = {features = ["serde"] }
}

#[derive(Queryable,Identifiable,Debug, Clone, Deserialize, Serialize)]
#[diesel(primary_key(id))]
#[diesel(table_name = sports)]
pub struct Sport {
    #[diesel(sql_type = Int4)]
    pub id: my_type_safety::Id,
    #[diesel(sql_type = Text)]
    pub name: String  
}

#[derive(Queryable,Identifiable,Associations,Debug, Clone, Deserialize, Serialize)]
#[diesel(primary_key(id_user, id_sport))]
#[diesel(table_name = user_sports)]
#[diesel(belongs_to(User, foreign_key = id_user))]
#[diesel(belongs_to(Sport, foreign_key = id_sport))]
pub struct UserSports {
    #[diesel(sql_type = Int4)]
    pub id_user: my_type_safety::Id,
    #[diesel(sql_type = Int4)]
    pub id_sport: my_type_safety::Id 
}

 
/*


#[derive(QueryableByName)]теперь может обрабатывать встраивание других структур. Иметь поле, тип которого является структурой, реализующей QueryableByName, а не один столбец в запросе, добавьте аннотацию #[diesel(embed)]
https://github.com/diesel-rs/diesel/blob/master/CHANGELOG.md#100---2018-01-02
*/
#![allow(unused_mut)]
#![allow(unused_variables)]
#![allow(unused_imports)]

pub use actix_diesel_actor_example as db;
use db::*; 
use db::AppState;
use db::models;
use db::SQuery;
use db::DbExecutorError;
 

use std::marker::PhantomData;
use actix_web::{get, web, App, HttpServer, Responder, HttpRequest, HttpResponse};
use diesel::prelude::*;
use chrono::prelude::*;

#[get("/hello/{name}")]
async fn greet(name: web::Path<String>) -> impl Responder {
    format!("Hello {name}!",name=name)
}

#[get("/dml_select")]
async fn dml_select(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, DbExecutorError> {
     
    use schema::users::dsl::{users,id as user_id};
   // schema::users::dsl::users::all_columns() вместо перечисления всех полей

   /*
   Trait diesel::prelude::QueryDsl
        count
        distinct
        distinct_on
        filter
        find
        for_key_share
        for_no_key_update
        for_share
        for_update
        group_by
        having
        inner_join
        into_boxed
        left_join
        left_outer_join
        limit
        no_wait
        nullable
        offset
        or_filter
        order
        order_by
        select
        single_value
        skip_locked
        then_order_by
   */
  // distinct()
  // SELECT DISTINCT "users"."name" FROM "users"
  //  let dist_name = schema::users::dsl::users.select(schema::users::dsl::name).distinct().load::<String>(connection)?;
  let query = schema::users::dsl::users.select(schema::users::dsl::name).distinct();
  let select:SQuery<_,String> = SQuery{ select: query, phantom: PhantomData::<String> };
  let usr_list:actix::prelude::Request<_,SQuery<_,String>> = data.rdb.send(select);
  let res_names:Vec<String> = usr_list.await??;
  
  // order_by()
  // distinct_on()
  // SELECT DISTINCT ON ("users"."name")"users"."name", "users"."email" FROM "users" ORDER BY "users"."name", "users"."email"
  let query = schema::users::dsl::users
    .select((schema::users::dsl::name, schema::users::dsl::email))
    .order_by((schema::users::dsl::name, schema::users::dsl::email))
    .distinct_on(schema::users::dsl::name);
  let select:SQuery<_,(String,String)> = SQuery{ select: query, phantom: PhantomData::<(String,String)> };
  let usr_list:actix::prelude::Request<_,SQuery<_,(String,String)>> = data.rdb.send(select);
  let res:Vec<(String,String)> = usr_list.await??;

   // ALL TABLE
   // SELECT * FROM users;
   // let all_users = schema::users::dsl::users.load::<(i32, String)>(connection)?;
   let query = schema::users::dsl::users;
   let select:SQuery<_,models::User> = SQuery{ select: query, phantom: PhantomData::<models::User> };
   let usr_list:actix::prelude::Request<_,SQuery<_,models::User>> = data.rdb.send(select);
   let all_users:Vec<models::User> = usr_list.await??;

    // SELECT users.name FROM users;
   // let all_names = schema::users::dsl::users.select(schema::users::dsl::name).load::<String>(conn)?;

   // left_join()
   // SELECT "users"."id", "users"."name", "users"."email", "users"."age", "users"."obj", "users"."create_at", "posts"."id", "posts"."id_user", "posts"."title", "posts"."create_at" FROM ("users" LEFT OUTER JOIN "posts" ON ("posts"."id_user" = "users"."id"))
   // schema::users::dsl::users.left_join(schema::posts::dsl::posts).load::< (models::User, Option<models::Post>) >(connection)?;
   let query = schema::users::dsl::users.left_join(schema::posts::dsl::posts);
   let select:SQuery<_,(models::User, Option<models::Post>)> = SQuery{ select: query, phantom: PhantomData::<(models::User, Option<models::Post>)> };
   let usr_list:actix::prelude::Request<_,SQuery<_,(models::User, Option<models::Post>)>> = data.rdb.send(select);
   let all_users:Vec<(models::User, Option<models::Post>)> = usr_list.await??;
  
  // left_join()
  // SELECT "users"."name", "posts"."title" FROM ("users" LEFT OUTER JOIN "posts" ON ("posts"."id_user" = "users"."id"))
  // let names_and_titles = schema::users::dsl::users.left_join(schema::posts::dsl::posts).select((users::name, posts::title.nullable())).load::<(String, Option<String>)>(connection)?;
   let query = schema::users::dsl::users.left_join(schema::posts::dsl::posts).select((schema::users::dsl::name, schema::posts::dsl::title.nullable()));
   let select:SQuery<_,(String, Option<String>)> = SQuery{ select: query, phantom: PhantomData::<(String, Option<String>)> };
   let usr_list:actix::prelude::Request<_,SQuery<_,(String, Option<String>)>> = data.rdb.send(select);
   let all_users:Vec<(String, Option<String>)> = usr_list.await??;
   
   // inner_join()
   // SELECT "users"."name", "posts"."title" FROM ("users" INNER JOIN "posts" ON ("posts"."id_user" = "users"."id"))
   let query = schema::users::dsl::users.inner_join(schema::posts::dsl::posts).select((schema::users::dsl::name, schema::posts::dsl::title));
   let select:SQuery<_,(String, String)> = SQuery{ select: query, phantom: PhantomData::<(String, String)> };
   let usr_list:actix::prelude::Request<_,SQuery<_,(String, String)>> = data.rdb.send(select);
   let all_users:Vec<(String, String)> = usr_list.await??;
   
    // inner_join() ON custom
   // SELECT "users"."name", "posts"."title" FROM ("users" INNER JOIN "posts" ON ("posts"."title" LIKE (("users"."name" || $1)))) -- binds: ["%"]
   let query = schema::users::dsl::users
   .inner_join(schema::posts::dsl::posts.on(schema::posts::dsl::title.like(schema::users::dsl::name.concat("%"))))
   .select((schema::users::dsl::name, schema::posts::dsl::title));
   // let data = schema::users::dsl::users.inner_join(schema::posts::dsl::posts.on(schema::posts::dsl::title.like(schema::users::dsl::name.concat("%")))).load::<(models::User, models::Post)>(connection);

   // left_join()
   // left_outer_join()
   // let names_and_titles = users::table.left_join(posts::table).select((users::name, posts::title.nullable())).load::<(String, Option<String>)>(conn)?;

   // filter() WHERE
   // let seans_id = users.filter(name.eq("Sean")).select(id).first(conn);

   // or_filter() WHERE OR
   // SELECT "users"."name" FROM "users" WHERE (("users"."name" = $1) OR ("users"."age" = $2)) -- binds: ["Fido", 18]
   let query = schema::users::dsl::users.filter(schema::users::dsl::name.eq("Fido")).or_filter(schema::users::dsl::age.eq(18)).select(schema::users::dsl::name);
   
   // find()
   // SELECT "users"."id", "users"."name", "users"."email", "users"."age", "users"."obj", "users"."create_at" FROM "users" WHERE ("users"."id" = $1)
   let query = schema::users::dsl::users.find(models::my_type_safety::Id(1));
 

   // order ORDER BY DESC/ASC
   // .order_by(foo).then_order_by(bar) эквивалентно .order((foo, bar))
   // let ordered_names = users.select(name).order(name.desc()).load::<String>(connection)?;

   // limit() LIMIT
   // offset()
   // let limited = users.select(name).order(id).limit(2).offset(1).load::<String>(connection)?;

   // group_by() GROUP BY
   // let data = users::table.inner_join(posts::table).group_by(users::id).select((users::name, count(posts::id))).load::<(String, i64)>(conn)?;
   // having()
   // let data = users::table.inner_join(posts::table).group_by(users::id).having(count(posts::id).gt(1)).select((users::name, count(posts::id))).load::<(String, i64)>(conn)?;

    // ---------------------------------------------------------------------------------------
    // Trait diesel::prelude::BelongingToDsl
    // Для реализующих Identifiable
    // belonging_to()
    // Построитель связи структур

    // SELECT "posts".* FROM "posts" WHERE (("posts"."id_user" = $1) AND ("posts"."id" = $2)) -- binds: [Id(2), Id(1)]
    let json = r#"{ "id":2, "name": "petr", "email": "petr@mail.com", "age": 18 }"#;
    let user:models::User = serde_json::from_str::<models::User>(json).unwrap();
    let post_id = models::my_type_safety::Id(1);
    //let user = schema::users::dsl::users.find(models::my_type_safety::Id(1));
    let query = models::Post::belonging_to(&user).find(post_id);//.first::<models::Post>(conn)?;
    
   
   // SELECT "sports"."name" 
   // FROM ("user_sports" INNER JOIN "sports" ON ("user_sports"."id_sport" = "sports"."id")) 
   // WHERE ("user_sports"."id_user" = $1) -- binds: [Id(2)]
    let query =  models::UserSports::belonging_to(&user)
     .inner_join(schema::sports::dsl::sports)
     .select(schema::sports::dsl::name);
     
    /*
    let comments = Comment::belonging_to(&posts)
    .inner_join(users::table)
    .select((comments::all_columns, (users::id, users::username)))
    .load::<(Comment, User)>(conn)?
    .grouped_by(&posts);
    */
 

  // ---------------------------------------------------------------------------------------
    /*
         schema::users::dsl::id  реализует:

        Trait diesel::expression_methods::ExpressionMethods:
            is_not_null
            asc
            between
            desc
            eq
            eq_any
            ge
            gt
            is_not_null
            is_null
            le
            lt
            ne
            ne_all
            not_between
    */
    // is_not_null() IS NOT NULL
    // SELECT "users"."name" FROM "users" WHERE ("users"."name" IS NOT NULL)
    let query = schema::users::dsl::users.select(schema::users::dsl::name).filter(schema::users::dsl::name.is_not_null());
    
    // is_null()  IS NULL
    // SELECT "users"."name" FROM "users" WHERE ("users"."name" IS NULL)
    // let data = users.select(name).filter(obj.is_null()).first::<String>(conn)?;

    // ne_all() NOT IN
    // eq_any() IN
    // SELECT "users"."id" FROM "users" WHERE ("users"."name" != ALL($1)) -- binds: [["Sean", "Jim"]]
    let query = schema::users::dsl::users.select(schema::users::dsl::id).filter(schema::users::dsl::name.ne_all(vec!["Sean", "Jim"]));
    // SELECT "users"."id" FROM "users" WHERE ("users"."name" = ANY($1))
    let query = schema::users::dsl::users.select(schema::users::dsl::id).filter(schema::users::dsl::name.eq_any(vec!["Sean", "Jim"]));

    // eq() =
    // ne() !=
    // gt() >
    // ge() >=
    // lt() <
    // le() <=
    // between() BETWEEN
    // not_between() NOT BETWEEN
    // desc() DESC
    // asc() ASC
    // let data = users.select(id).filter(name.eq("Sean"));
    // let data = users.select(id).filter(name.ne("Sean"));
    // let data = users.select(name).filter(id.gt(1)).first::<String>(connection)?;
    // let data = users.select(name).filter(id.ge(2)).first::<String>(connection)?;
    // let data = users.select(name).filter(id.lt(2)).first::<String>(connection)?;
    // let data = users.select(name).filter(age.between(18, 26)).first(connection)?;
    // let names = users.select(name).order(name.desc()).load::<String>(connection)?;

    // ---------------------------------------------------------------------------------------
    /*
    Для работы diesel::sql_query() нужна реализация QueryableByName
    #[derive(Queryable, QueryableByName)]
    struct User{...}

    Struct diesel::query_builder::SqlQuery
    - bind()
    - into_boxed()
    - sql()

        // into_boxed()
        let mut query = diesel::sql_query("SELECT * FROM users WHERE id > $1 AND name <> $2").into_boxed();
        let mut query = query.bind::<diesel::sql_types::Int4, _>(1);
        let query = query.bind::<diesel::sql_types::Text, _>("petr");
        let users:Vec<models::User> = query.get_results(conn).unwrap();

    */
    // SELECT * FROM users WHERE id > $1 AND name = $2 -- binds: [1, "petr"]
     let query = diesel::sql_query("SELECT * FROM users WHERE id > $1 AND name = $2")
        .bind::<diesel::sql_types::Int4/*schema::custom_sql_types::Id*/, _>(1/*models::my_type_safety::Id(1)*/)
        .bind::<diesel::sql_types::Text, _>("petr");
    let select:SQuery<_,models::User> = SQuery{ select: query, phantom: PhantomData::<models::User> };
    let usr_list:actix::prelude::Request<_,SQuery<_,models::User>> = data.rdb.send(select);
    let res_users:Vec<models::User> = usr_list.await??;

    // Function diesel::dsl::sql
    // SELECT "users"."id", "users"."name", "users"."email", "users"."age", "users"."obj", "users"."create_at" FROM "users" WHERE id > $1 AND name = $2 -- binds: [1, "petr"]
    let query = schema::users::dsl::users
    .select((schema::users::dsl::id,schema::users::dsl::name,schema::users::dsl::email,schema::users::dsl::age,schema::users::dsl::obj,schema::users::dsl::create_at))
    .filter(
        diesel::dsl::sql::<diesel::sql_types::Bool>("id > ")
        .bind::<diesel::sql_types::Integer,_>(1)
        .sql(" AND name = ")
        .bind::<diesel::sql_types::Text, _>("petr")
    );
    let select:SQuery<_,models::User> = SQuery{ select: query, phantom: PhantomData::<models::User> };
    let usr_list:actix::prelude::Request<_,SQuery<_,models::User>> = data.rdb.send(select);
    let res_users:Vec<models::User> = usr_list.await??;
   // ---------------------------------------------------------------------------------------
   // Function diesel::dsl::not
   // Function diesel::dsl::max, Function diesel::dsl::min, Function diesel::dsl::avg
   // Function diesel::dsl::select
   // Function diesel::dsl::date
   // Function diesel::dsl::count_star, Function diesel::dsl::count_distinct, Function diesel::dsl::count
   // Trait diesel::expression_methods::BoolExpressionMethods and(), or()
   /*
    let id = schema::users::dsl::users.select(diesel::dsl::max(schema::users::dsl::id))
    .filter(
        diesel::dsl::not(schema::users::dsl::name.eq("brbrb"))
     ).first(connection).unwrap();


     // SELECT date(CURRENT_TIMESTAMP) 
     let today = diesel::select(diesel::dsl::date(diesel::dsl::now)).first(connection)?;

     // SELECT COUNT(*) FROM "users"
     let count = schema::users::dsl::users.count().get_result(conn);
     let count = schema::users::dsl::users.select(diesel::dsl::count_star()).first(conn);

    */
    
     // min().max(),avg(),and()
    // SELECT max("users"."create_at"), min("users"."age"), avg("users"."age") FROM "users" WHERE  NOT ((("users"."name" = $1) AND ("users"."name" = $2))) -- binds: ["brbrb", "brbrb"]
    let query = schema::users::dsl::users.select((
                                    diesel::dsl::max(schema::users::dsl::create_at), 
                                    diesel::dsl::min(schema::users::dsl::age),
                                    diesel::dsl::avg(schema::users::dsl::age)
                                ))
    .filter(
        diesel::dsl::not(schema::users::dsl::name.eq("brbrb").and(schema::users::dsl::name.eq("brbrb"))),
    );
   
    println!(">>>{}",  diesel::debug_query::<diesel::pg::Pg, _>(&query));
    // SELECT EXISTS (SELECT "users"."id", "users"."name", "users"."email", "users"."age", "users"."obj", "users"."create_at" FROM "users" WHERE ("users"."name" = $1)) -- binds: ["Jim"]
    let query = diesel::dsl::select(diesel::dsl::exists(schema::users::dsl::users.filter(schema::users::dsl::name.eq("Jim"))));

    // ---------------------------------------------------------------------------------------
    // SELECT "users"."id", "users"."name", "users"."email", "users"."age", "users"."obj", "users"."create_at" FROM "users" WHERE ("users"."id" IS NOT NULL)
    let query = schema::users::dsl::users.filter(user_id.is_not_null());
    let select:SQuery<_,models::User> = SQuery{ select: query, phantom: PhantomData::<models::User> };
    let usr_list:actix::prelude::Request<_,SQuery<_,models::User>> = data.rdb.send(select);
    let res_users:Vec<models::User> = usr_list.await??;
    //let res_users:String = serde_json::to_string(&users).unwrap();
    
     Ok( 
     HttpResponse::Ok()
        .content_type(actix_web::http::header::ContentType::json())
        .json(res_users))

     //Ok(HttpResponse::Found().finish())
}


#[get("/dml_insert")]
async fn dml_insert(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, DbExecutorError> {
     /*
         users => schema::users::dsl::users or schema::users::table

        .execute() - выполнить запрос
        .get_results() - выполнить запрос и вернуть все вставленные строки
        .get_result() - выполнить запрос и вернуть вставленную строку
        .returning() - вернуть определенное поле из результата (не выполняет запрос)

       Если все поля таблици имеют DEFAULT:
            INSERT INTO "users" DEFAULT VALUES
            insert_into(schema::users::dsl::users or schema::users::table).default_values().execute(conn)

            INSERT INTO "users" ("name") VALUES ($1)
            insert_into(schema::users::dsl::users or schema::users::table).values(schema::users::dsl::name.eq("Sean")).execute(conn)
           
            INSERT INTO "users" ("name","age") VALUES ($1,$2)
            insert_into(schema::users::dsl::users or schema::users::table).values((schema::users::dsl::name.eq("Sean"),schema::users::dsl::age.eq(18))).execute(conn)
     
            Пакетная вставка:
            INSERT INTO "users" ("name","email","age") VALUES ($1, $2. $3), ($4, $5, $6)
                insert_into(users)
                .values(&vec![
                    (schema::users::dsl::name.eq("Sean"), schema::users::dsl::email.eq("Sean@mail.com"), schema::users::dsl::age.eq(18)),
                    (schema::users::dsl::name.eq("Tess"), schema::users::dsl::email.eq("Tess@mail.com"), schema::users::dsl::age.eq(18)),
                ])
                .execute(conn)

           // returning()
           let id:Id = insert_into(schema::users::dsl::users or schema::users::table)
           .values(schema::users::dsl::name.eq("Sean"),schema::users::dsl::email.eq("Sean@mail.com"), schema::users::dsl::age.eq(18), None, None)
           .returning(schema::users::dsl::id)
           .get_result(conn);

       Trait diesel::prelude::Insertable:
            #[derive(Insertable)] struct NewUser{...}
            let json = r#"{ "name": "Sean", "email": "petr@mail.com", "age": 18 }"#;
            let new_user:models::NewUser = serde_json::from_str::<models::NewUser>(json).unwrap();
            insert_into(schema::users::table).values(&new_user).execute(conn)?;

            Пакетная вставка:
            INSERT INTO "users" ("name","email","age") VALUES ($1, $2. $3), ($4, $5, $6)
            let json = r#"[
                { "name": "Sean", "email": "Sean@mail.com", "age": 18 },
                { "name": "Tess", "email": "Tess@mail.com", "age": 18 }
            ]"#;
            let new_user = serde_json::from_str::<Vec<NewUser>>(json)?;
            insert_into(schema::users::table).values(&new_user).execute(conn)?;


       Struct diesel::query_builder::InsertStatement
            - into_columns() - При вставке из оператора select список столбцов можно указать с помощью .into_columns
            - on_conflict() - Добавляет ON CONFLICT, если возникает конфликт для заданного уникального ограничения поля
            - on_conflict_do_nothing() - Добавляет ON CONFLICT DO NOTHING (не выдавать ошибку)
            - returning() - вернуть определенное поле из результата (не выполняет запрос)

            // into_columns()
            let new_posts = schema::users::table.select(( users::name.concat("'s First Post"), users::id));
            diesel::insert_into(posts::table)
                .values(new_posts)
                .into_columns((posts::title, posts::user_id))
                .execute(conn)?;
           
            // on_conflict_do_nothing()
            let inserted_row_count:QueryResult<usize> = diesel::insert_into(schema::users::table)
                .values(&new_user)
                .on_conflict_do_nothing()
                .execute(conn);
            
            // on_conflict()
            let inserted_row_count:QueryResult<usize> = diesel::insert_into(schema::users::table)
                .values(&new_user)
                .on_conflict(schema::users::dsl::name)
                .execute(conn);
           // Для составного индекса уникальности `CREATE UNIQUE INDEX users_name_age ON users (name, age)`
           // .on_conflict((schema::users::dsl::name, schema::users::dsl::age))
 
        Function diesel::dsl::insert_or_ignore_into:
           only MySQL, SQLite   

        Struct diesel::upsert::IncompleteOnConflict:
          - do_update() ON CONFLICT (...) DO UPDATE ...
            // если был конфлик по id то выполнить update с другим name
            let insert_count = diesel::insert_into(schema::users::dsl::users)
                .values(&new_user)
                .on_conflict(id)
                .do_update()
                .set(name.eq("I DONT KNOW ANYMORE"))
                .execute(conn);
             // если был конфлик по id то выполнить update с другим NewUser
            let insert_count = diesel::insert_into(schema::users::dsl::users)
                .values(&new_user)
                .on_conflict(id)
                .do_update()
                .set(&new_user_2)
                .execute(conn);

        diesel::upsert::excluded() - получить отклоненное значение
        // если был конфлик по id то выполнить update с полученными данными от excluded()
        let insert_count = diesel::insert_into(schema::users::dsl::users)
            .values(&vec![new_user_1, new_user_2])
            .on_conflict(schema::users::dsl::id)
            .do_update()
            .set(name.eq(excluded(name)))
            .execute(conn);

     */
   
    let data_json = r#"
        {
            "name": "John Doe",
            "age": 43,
            "phones": [
                "+44 1234567",
                "+44 2345678"
            ]
        }"#;
    let new_user = models::NewUser {
        name: "petr",
        email: "petr@mail.com",
        age: 18_i32,
        obj: Some(serde_json::from_str(data_json).unwrap()),
        create_at: Some(Utc::now().naive_utc())
    };
    /*
    let user:models::User = diesel::insert_into(schema::users::table)
        .values(new_user);
        .get_result(conn)
        .expect("Error saving new post")
    */
    // INSERT INTO "users" ("name", "email", "age", "obj", "create_at") VALUES ($1, $2, $3, $4, $5)
    let query = diesel::insert_into(schema::users::dsl::users/* or schema::users::table*/).values(new_user);
    println!("{}",  diesel::debug_query::<diesel::pg::Pg, _>(&query));
    let insert:WQuery<_, models::User> = WQuery{ query: query, phantom: PhantomData::<models::User> };
    let res:actix::prelude::Request<_,WQuery<_,models::User>> = data.wdb.send(insert);
    let user:Vec<models::User> = res.await??;
      
    Ok( 
        HttpResponse::Ok()
           .content_type(actix_web::http::header::ContentType::json())
           .json(user))
}

#[get("/dml_update")]
async fn dml_update(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, DbExecutorError> {
    use schema::users::dsl::{users, id, age, email, name , obj, create_at};
    
    let data_age:i32 = 19;
    let data_email:String = "petr@gmail.com".to_owned();
    let id_find:models::my_type_safety::Id = models::my_type_safety::Id(1);
    /*
      Типы для target update:
           Trait  diesel::query_builder::IntoUpdateTarge:
             - let target = schema::users::dsl::users; 
             - let target = schema::users::dsl::users.filter(create_at.lt(diesel::dsl::now));
             - let target = schema::users::dsl::users.find(id_find);
           Trait diesel::prelude::Identifiable: 
               #[derive(Identifiable)] 
               struct User{pub id:i32}
             - let target:User = User{id:1};
           Trait diesel::prelude::AsChangeset:
                #[derive(AsChangeset)]
                struct User{...}
             - let target:User = User{id:1,...};
               diesel::update(schema::users::table).set(&target)
           Eсли ваша структура имеет оба #[derive(AsChangeset)] и #[derive(Identifiable)]:
             - schema::users::dsl::users.save_changes(&conn) вместо diesel::update(schema::users::table).set(&target).get_result(&conn)

     let upd:diesel::query_builder::UpdateStatement<...> =  diesel::update(target);
     Struct diesel::query_builder::UpdateStatement:
        - filter()
        - into_boxed() (for build WHERE)
        - returning()
        - set()

        let query = upd.set(age.eq(data_age));
        query Результирующий оператор затем запускается путем вызова либо `execute`, `get_result`, или `get_results`
        (для возврата 0 или 1 => .get_result(...).optional();) 
    
    let user = diesel::update(schema::users::dsl::users.find(id_find))
        .set(age.eq(data_age))
        .get_result::<models::User>(&conn)
        .expect(&format!("Unable to find post {}", id_find));

    let updated_rows = diesel::update(schema::users::dsl::users.find(id_find))
        .set((age.eq(data_age),email.eq(data_email)))
        .filter(name.eq("petr"))
        .execute(conn);


   Function diesel::dsl::replace_into REPLACE:
        diesel::dsl::replace_into(schema::users::dsl::users)
            .values((schema::users::dsl::id.eq(1), schema::users::dsl::name.eq("Jim")))
            .execute(conn)
            .unwrap();
    */
    
     // --------------------------------------------------------------------------------------
     // returning()
     let query = diesel::update(schema::users::dsl::users.filter(id.eq(id_find)))
                              .set(obj.eq(serde_json::json!({})))
                              .returning(obj);
     let update:WQuery<_, Option<serde_json::Value>> = WQuery{ query: query, phantom: PhantomData::< Option<serde_json::Value>> };
     let res:actix::prelude::Request<_,WQuery<_, Option<serde_json::Value>>> = data.wdb.send(update);
     let ret_obj:Vec<Option<serde_json::Value>> = res.await??;

     // --------------------------------------------------------------------------------------
     // UPDATE `users` SET `age` = `users`.`age` + 1
     let query = diesel::update(schema::users::dsl::users).set(age.eq(age + 1));
     println!("{}",  diesel::debug_query::<diesel::pg::Pg, _>(&query));
     let update:WQuery<_, models::User> = WQuery{ query: query, phantom: PhantomData::<models::User> };
     let res:actix::prelude::Request<_,WQuery<_,models::User>> = data.wdb.send(update);
     let _user:Vec<models::User> = res.await??;
     // --------------------------------------------------------------------------------------
     // UPDATE `users` SET `age` =  data_age WHERE create_at < CURRENT_TIMESTAMP AND name = 'petr'
     let target = schema::users::dsl::users.filter(create_at.lt(diesel::dsl::now));
     let query =  diesel::update(target)
                 .set(age.eq(data_age))
                 .filter(name.eq("petr"));
     println!("{}",  diesel::debug_query::<diesel::pg::Pg, _>(&query));
     let update:WQuery<_, models::User> = WQuery{ query: query, phantom: PhantomData::<models::User> };
     let res:actix::prelude::Request<_,WQuery<_,models::User>> = data.wdb.send(update);
     let _user:Vec<models::User> = res.await??;

     // ---------------------------------------------------------------------------------------
     // into_boxed()
     // UPDATE `users` SET `age` =  data_age, email = data_email WHERE id = id_find
     let query = diesel::update(schema::users::dsl::users.find(id_find))
                        .set((age.eq(data_age),email.eq(data_email.clone())))
                        .filter(name.eq("petr"));
        // or build WHERE into_boxed()
        let mut query = diesel::update(schema::users::dsl::users.find(id_find))
                            .set((age.eq(data_age),email.eq(data_email))).into_boxed();
        let query = query.filter(name.eq("petr"));

     println!("{}",  diesel::debug_query::<diesel::pg::Pg, _>(&query));    
     let update:WQuery<_, models::User> = WQuery{ query: query, phantom: PhantomData::<models::User> };
     let res:actix::prelude::Request<_,WQuery<_,models::User>> = data.wdb.send(update);
     let res_user:Vec<models::User> = res.await??;
     // --------------------------------------------------------------------------------------
        // impl Identifiable
        let target:models::User =  res_user.first().unwrap().clone();
        let query =  diesel::update(target)
                    .set(age.eq(data_age))
                    .filter(name.eq("petr"));
        let update:WQuery<_, models::User> = WQuery{ query: query, phantom: PhantomData::<models::User> };
        let res:actix::prelude::Request<_,WQuery<_,models::User>> = data.wdb.send(update);
        let _user:Vec<models::User> = res.await??;
      
     // --------------------------------------------------------------------------------------
        // impl AsChangeset
        // UPDATE `users` SET `age` =  target.age, `email` = target.email,..... WHERE id = target.id
        let mut target:models::User =  res_user.first().unwrap().clone();
        target.age = 22;
        let query = diesel::update(schema::users::table).set(target);
        let update:WQuery<_, models::User> = WQuery{ query: query, phantom: PhantomData::<models::User> };
        let res:actix::prelude::Request<_,WQuery<_,models::User>> = data.wdb.send(update);
        let user:Vec<models::User> = res.await??;

        Ok( 
        HttpResponse::Ok()
            .content_type(actix_web::http::header::ContentType::json())
            .json(user))
}

#[get("/dml_delete")]
async fn dml_delete(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, DbExecutorError> {
    
    /*
     Struct diesel::query_builder::DeleteStatement:
        filter
        into_boxed
        or_filter
        returning
    */
    
    let id_find:models::my_type_safety::Id = models::my_type_safety::Id(2);
    
    // DELETE  FROM "posts" WHERE ("posts"."id" = $1) -- binds: [Id(2)]
    // diesel::delete(schema::posts::dsl::posts.filter(schema::posts::dsl::id.eq(2))).execute(conn)?;
    let query = diesel::delete(schema::posts::dsl::posts.filter(schema::posts::dsl::id.eq(id_find)));  
  

    // DELETE  FROM "posts"
    // diesel::delete(posts).execute(conn)?;
    let query = diesel::delete(schema::posts::dsl::posts);

    // filter()
    // or_filter()
    // DELETE  FROM "posts" WHERE ("posts"."title" LIKE $1) -- binds: ["%title2%"]
    // let deleted_rows = diesel::delete(schema::posts::dsl::posts).filter(schema::posts::dsl::title.like(pattern)).execute(conn);
    let pattern = format!("%{}%", "title2");
    let query = diesel::delete(schema::posts::dsl::posts.filter(schema::posts::dsl::title.like(&pattern)));
    // or
    let query = diesel::delete(schema::posts::dsl::posts).filter(schema::posts::dsl::title.like(&pattern));
    // DELETE  FROM "posts" WHERE ("posts"."title" LIKE $1 OR "posts"."title" LIKE $1) -- binds: ["%title2%","%title2%"]
    let pattern = format!("%{}%", "title2");
    let query = diesel::delete(schema::posts::dsl::posts).filter(schema::posts::dsl::title.like(&pattern)).or_filter(schema::posts::dsl::title.like(&pattern));
    
    // returning()
    // DELETE  FROM "posts" WHERE ("posts"."title" LIKE $1) RETURNING "posts"."id", "posts"."title" -- binds: ["%title2%"]
    let query = diesel::delete(schema::posts::dsl::posts.filter(schema::posts::dsl::title.like(pattern))).returning((schema::posts::dsl::id,schema::posts::dsl::title));
    println!("{}",  diesel::debug_query::<diesel::pg::Pg, _>(&query)); 
    let delete:WQuery<_, (models::my_type_safety::Id,String)> = WQuery{ query: query, phantom: PhantomData::<(models::my_type_safety::Id,String)> };
    let res:actix::prelude::Request<_,WQuery<_,(models::my_type_safety::Id,String)>> = data.wdb.send(delete);
    let res_delete:Vec<(models::my_type_safety::Id,String)> = res.await??;
    Ok( 
        HttpResponse::Ok()
            .content_type(actix_web::http::header::ContentType::json())
            .json(res_delete))
    // Ok(HttpResponse::Found().finish())
}

#[get("/")]
async fn index(req: HttpRequest, data: web::Data<AppState>) -> Result<HttpResponse, DbExecutorError> {
   
    let url = "http://localhost:4000";
    let mut body:String = "<html><body><h1>Test query</h1>".to_owned();
    body.push_str(&format!("<div> <a href='{url}/{param}'> {title} </a> </div>",url=url,param="dml_select",title="DML Select"));
    body.push_str(&format!("<div> <a href='{url}/{param}'> {title} </a> </div>",url=url,param="dml_insert",title="DML Insert"));
    body.push_str(&format!("<div> <a href='{url}/{param}'> {title} </a> </div>",url=url,param="dml_update",title="DML Update"));
    body.push_str(&format!("<div> <a href='{url}/{param}'> {title} </a> </div>",url=url,param="dml_delete",title="DML Delete"));
    body.push_str("</body></html>");
    Ok( 
        HttpResponse::Ok()
           .content_type(actix_web::http::header::ContentType::html())
           .body(body))
}

#[actix_web::main] // or #[tokio::main]
async fn main() -> std::io::Result<()>{
  
    let raddr = db::db_setup(db::ConnectionType::Read);
    let waddr = db::db_setup(db::ConnectionType::Write);
  
    HttpServer::new(move || {
            App::new()
            .app_data(web::Data::new(db::AppState{rdb: raddr.clone(), wdb: waddr.clone()})) 
            .route("/hello", web::get().to(|| async { "Hello World!" }))
            .service(greet)
            .service(index)
            .service(dml_select)
            .service(dml_insert)
            .service(dml_update)
            .service(dml_delete)

    })
    .bind(("127.0.0.1", 4000))?
    .run()
    .await
   
}
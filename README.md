# Actix Diesel Actor


> Разделение запросов выборки или изменения в разные пулы. Предоставляет типы сообщений для упаковки запросов Diesel в сообщения actix.
Предоставляет субъекты для обработки этих сообщений и выполнения запросов.

Сообщение SQuery можно использовать для всего, что является полным дизельным запросом и реализует дизельный SelectQuery.
Это полезно, когда ваше приложение выполняет операции чтения в реплике вашей базы данных, доступной только для чтения.
WQuery принимает операции записи, такие как INSERT, UPDATE, DELETE, в дополнение к SELECT.
Это можно использовать для общих целей или когда в вашем кластере БД есть много реплик и одна главная БД с возможностью записи.


## Links 

## Diesel tutorial
http://diesel.rs/guides/
https://diesel.rs/guides/getting-started

## Diesel project
https://github.com/Jekshmek/actix_db/blob/master/actix-diesel-actor/src/lib.rs
/home/jeka/Projects/Rust/2020-fall-bug
/home/jeka/Projects/Rust/server-diesel
https://github.com/Jekshmek/example-actix-web-juniper-diesel
https://github.com/diesel-rs/diesel/tree/master/examples/postgres

## Diesel doc
https://docs.rs/diesel/2.0.0-rc.0/diesel/index.html

## Actix web
https://actix.rs/docs/application/
https://docs.rs/actix-web/latest/actix_web/struct.HttpResponseBuilder.html#method.json

----------------------------------------------------------------------------------

## Установка зависимостей postgres

```
$ sudo apt-get install -y libpq-dev
```

## Установка интрументов командной строки CLI для управления через консоль базой postgres
```
$ cargo install diesel_cli --no-default-features --features postgres
```
## Migrations
    1.Файл .env
    postgres://username:password@host:port/name_db
    echo DATABASE_URL=postgres://username:password@localhost/diesel_demo > .env

    2.Создаст таблицу и папку для миграций
        $ diesel setup

    3.Создание файлов миграции
        Полный вариант 
         $ diesel migration --config-file diesel.toml  --migration-dir migrations --database-url postgres://game:game@localhost:5432/gamedb  generate create_users
        Короткий вариант
         $ diesel migration generate create_users

            Creating migrations/2022-05-15-095020_create_users/up.sql
            Creating migrations/2022-05-15-095020_create_users/down.sql

    4.Запускает все ожидающие миграции
        $ diesel migration run

    Отменяет и повторно запускает последнюю миграцию
        $ diesel migration redo
    
    Отменяет последнюю миграцию запуска
        $ diesel migration revert

    Список существующих миграций
         $ diesel migration list

         Migrations:
            [X] 00000000000000_diesel_initial_setup
            [X] 2022-05-16-171428_create_users
            [X] 2022-05-17-163725_create_posts
            [ ] 2022-05-18-082557_create_cars
            [ ] 2022-05-18-082612_create_sports
            [ ] 2022-05-18-082619_create_user_sports
    
## Migrations [diesel_migrations trait]    
[diesel_migrations trait](https://docs.rs/diesel_migrations/latest/diesel_migrations/macro.embed_migrations.html)


## Traits:
    https://github.com/diesel-rs/diesel/blob/master/guide_drafts/trait_derives.md

    Queryable - Типы, которые реализуют Queryable, представляют результат SQL-запроса.

    QueryableByName - Добавление QueryableByName к вашей структуре означает, что ее можно будет построить из результата необработанного 
                      SQL-запроса с использованием sql_query функции.

    Identifiable - позволяет использовать обьект структуры в качестве обьекта для поиска по его полю id для обновления полей таблицы
                   update(&your_struct) эквивалентно update(YourStruct::table().find(&your_struct.primary_key())

    Insertable - Представляет, что структура может использоваться для вставки новой строки в базу данных. 

    AsChangeset - По умолчанию AsChangeset предполагается, что каждый раз, когда поле имеет значение None, мы не хотим присваивать ему какие-либо значения (игнорировать). 
                  Если мы действительно хотим присвоить NULLзначение, мы можем использовать аннотацию #[diesel(treat_none_as_null = true)]

    Associations - запрос связанных записей  запрос связанных записей #[diesel(belongs_to(ParentStruct, foreign_key = my_custom_key))]

-------------------
 
Посмотреть SQL
println!("{}", debug_query::<Pg, _>(&our_query));

schema::users::table или schema::users::dsl::users тоже самое
-------------------

PG Отношения

 
Один ко многим

CREATE TABLE IF NOT EXISTS users( 
    id SERIAL PRIMARY KEY, 
    name VARCHAR(30) NOT NULL CHECK(name !=''), 
    email VARCHAR(30) NOT NULL UNIQUE CHECK(email !=''),
    age INTEGER  NOT NULL DEFAULT 18 CHECK(age >0 AND age < 100),
    obj JSON,
    price MONEY, 
    create_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS posts( 
    id SERIAL PRIMARY KEY, 
    id_user SERIAL, 
    title TEXT NOT NULL,
    create_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (id_user) 
    REFERENCES users(id) 
    ON UPDATE SET NULL 
    ON DELETE SET NULL      
);

INSERT INTO users(id,name,email,obj,price) VALUES (1,'petr','mail@petr.com','{}',0),(2,'kot','mail@kot.com','{}',0);
INSERT INTO posts(id_user,title) VALUES (1,'title1'),(1,'title2');

SELECT p.title
FROM posts p INNER JOIN user u ON u.id=p.id_user
----------------
Один к одному

CREATE TABLE IF NOT EXISTS users( 
    id SERIAL PRIMARY KEY, 
    name VARCHAR(30) NOT NULL CHECK(name !=''), 
    email VARCHAR(30) NOT NULL UNIQUE CHECK(email !=''),
    age INTEGER  NOT NULL DEFAULT 18 CHECK(age >0 AND age < 100),
    obj JSON,
    price MONEY, 
    create_at TIMESTAMP DEFAULT NOW()
);

CREATE TABLE IF NOT EXISTS cars( 
    id SERIAL PRIMARY KEY, 
    id_user SERIAL UNIQUE, 
    car TEXT NOT NULL,
    description TEXT,
    create_at TIMESTAMP DEFAULT NOW(),
    FOREIGN KEY (id_user) 
    REFERENCES users(id) 
    ON UPDATE SET NULL 
    ON DELETE SET NULL      
);

INSERT INTO users(id,name,email,obj,price) VALUES (1,'petr','mail@petr.com','{}',0),(2,'kot','mail@kot.com','{}',0);
INSERT INTO posts(id_user,title) VALUES (1,'title1'),(2,'title2');
------------------

Многие ко многим
 
CREATE TABLE IF NOT EXISTS sports(
    id SERIAL PRIMARY KEY,
    name varchar(255) NOT NULL  
);

 CREATE TABLE IF NOT EXISTS user_sports(
    id_user SERIAL REFERENCES users ON DELETE CASCADE ON UPDATE CASCADE,
    id_sport SERIAL REFERENCES sports ON DELETE CASCADE ON UPDATE CASCADE,
    PRIMARY KEY (id_user, id_sport) 
);

 
INSERT INTO sports(name) VALUES('box'),('ping-pong'),('run'),('gum'); 
INSERT INTO user_sports(id_user,id_sport) VALUES (1,1), (1,2),(2,1),(2,2),(2,3),(2,4);  

SELECT s.name
FROM sports s
INNER JOIN user_sports us ON us.id_sport=s.id
INNER JOIN users u ON u.id=us.id_user
WHERE u.name='petr'
 
-----------------------------------------------------
Операторы SQL делятся на:

    операторы определения данных (Data Definition Language, DDL):
        CREATE создаёт объект базы данных (саму базу, таблицу, представление, пользователя и так далее),
        ALTER изменяет объект,
        DROP удаляет объект;
    операторы манипуляции данными (Data Manipulation Language, DML):
        SELECT выбирает данные, удовлетворяющие заданным условиям,
        INSERT добавляет новые данные,
        UPDATE изменяет существующие данные,
        DELETE удаляет данные;
    операторы определения доступа к данным (Data Control Language, DCL):
        GRANT предоставляет пользователю (группе) разрешения на определённые операции с объектом,
        REVOKE отзывает ранее выданные разрешения,
        DENY задаёт запрет, имеющий приоритет над разрешением;
    операторы управления транзакциями (Transaction Control Language, TCL):
        COMMIT применяет транзакцию,
        ROLLBACK откатывает все изменения, сделанные в контексте текущей транзакции,
        SAVEPOINT делит транзакцию на более мелкие участки.
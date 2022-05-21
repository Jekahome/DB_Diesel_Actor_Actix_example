-- Your SQL goes here
CREATE TABLE IF NOT EXISTS users( 
    id SERIAL PRIMARY KEY, 
    name VARCHAR(30) NOT NULL CHECK(name !=''), 
    email VARCHAR(30) NOT NULL UNIQUE CHECK(email !=''),
    age INTEGER  NOT NULL DEFAULT 18 CHECK(age >0 AND age < 100),
    obj JSON,
    create_at TIMESTAMP DEFAULT NOW()
);
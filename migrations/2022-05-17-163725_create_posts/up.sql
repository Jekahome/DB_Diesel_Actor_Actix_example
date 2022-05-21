-- Your SQL goes here

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
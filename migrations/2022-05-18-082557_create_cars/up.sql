-- Your SQL goes here
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
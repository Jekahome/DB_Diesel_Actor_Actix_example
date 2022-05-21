
extern crate diesel_migrations;
 
use diesel::prelude::Connection;// for establish()
use diesel::pg::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

fn run_migrations(connection: &mut impl MigrationHarness<diesel::pg::Pg>) -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    connection.run_pending_migrations(MIGRATIONS)?;
    Ok(())
}

fn main() {
    let database_url = std::env::var("DB_WRITE_URL").expect("DB_WRITE_URL must be set");
    let mut conn:PgConnection = PgConnection::establish(&database_url)
                .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));
    run_migrations(&mut conn).unwrap();
}
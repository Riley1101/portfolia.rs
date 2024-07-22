use core::fmt::{Display, Formatter};
use diesel::pg::PgConnection;
use diesel::r2d2::ConnectionManager;
use diesel::r2d2::Pool;
use dotenvy::dotenv;
use std::{env, fmt};

pub fn get_connection_pool() -> Pool<ConnectionManager<PgConnection>> {
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    // Refer to the `r2d2` documentation for more methods to use
    // when building a connection pool
    Pool::builder()
        .test_on_check_out(true)
        .build(manager)
        .expect("Could not build connection pool")
}

pub struct DatabaseConfig {
    pub pool: Pool<ConnectionManager<PgConnection>>,
}

impl Display for DatabaseConfig {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "DbConnection")
    }
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        Self::new()
    }
}

impl DatabaseConfig {
    pub fn new() -> Self {
        DatabaseConfig {
            pool: get_connection_pool(),
        }
    }
}

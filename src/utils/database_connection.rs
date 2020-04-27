use diesel::pg::PgConnection;
use diesel::r2d2::{Pool, PooledConnection, ConnectionManager};
use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

type PostgresPool = Pool<ConnectionManager<PgConnection>>;
pub type PostgresPooledConnection = PooledConnection<ConnectionManager<PgConnection>>;

lazy_static! {
    pub static ref POOL: PostgresPool = { init_pool() };
}

fn init_pool() -> PostgresPool {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let manager = ConnectionManager::<PgConnection>::new(database_url);
    Pool::builder()
        .build(manager)
        .expect("Failed to create pool.")
}

pub fn get_pooled_connection() -> PostgresPooledConnection {
    let pool = POOL.clone();
    let connection = pool.get().expect("Failed to get pooled connection");
    connection
}

pub fn get_test_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = PgConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url));
    connection.begin_test_transaction().unwrap();
    connection
}

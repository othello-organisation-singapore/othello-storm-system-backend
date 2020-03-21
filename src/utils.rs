use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

pub struct ExternalServices {
    pub connection: PgConnection
}

impl ExternalServices {
    pub fn get_connection(&self) -> &PgConnection {
        &self.connection
    }
}

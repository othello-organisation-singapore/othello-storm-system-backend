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
    pub fn create_live_services() -> ExternalServices {
        let connection = establish_connection();
        ExternalServices {
            connection
        }
    }

    pub fn create_test_services() -> ExternalServices {
        let connection = establish_connection();
        let _ = connection.begin_test_transaction();
        ExternalServices {
            connection
        }
    }

    pub fn get_connection(&self) -> &PgConnection {
        &self.connection
    }
}

use diesel::prelude::*;
use rocket_contrib::json::{Json, JsonValue};

use crate::errors::ErrorType;

pub trait ResponseCommand {
    fn execute(&self, connection: &PgConnection) -> Json<JsonValue> {
        match self.do_execute(connection) {
            Ok(result) => {
                info!("Successful request for {}", self.get_request_summary());
                Json(json!({
                    "success": result,
                    "error": {
                        "code": 0,
                        "message": "",
                    }
                }))
            }
            Err(error) => {
                error!(
                    "Failed request for {}, {}",
                    self.get_request_summary(),
                    &error.to_error_message()
                );
                Json(json!({
                    "success": "",
                    "error": {
                        "code": &error.to_error_code(),
                        "message": &error.to_error_message(),
                    }
                }))
            }
        }
    }

    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, ErrorType>;

    fn get_request_summary(&self) -> String;
}

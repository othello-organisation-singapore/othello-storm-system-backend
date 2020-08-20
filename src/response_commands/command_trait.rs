use diesel::prelude::*;
use rocket_contrib::json::{Json, JsonValue};

pub trait ResponseCommand {
    fn execute(&self, connection: &PgConnection) -> Json<JsonValue> {
        match self.do_execute(connection) {
            Ok(result) => Json(json!({
                "success": result,
                "error": ""
            })),
            Err(message) => {
                error!("Failed request, {}", &message);
                Json(json!({
                    "success": "",
                    "error": message
                }))
            }
        }
    }

    fn do_execute(&self, connection: &PgConnection) -> Result<JsonValue, String>;
}

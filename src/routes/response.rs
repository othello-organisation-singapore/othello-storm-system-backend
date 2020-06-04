use rocket_contrib::json::{Json, JsonValue};

pub fn create_response(process_result: Result<JsonValue, String>) -> Json<JsonValue> {
    match process_result {
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
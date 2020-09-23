use mocktopus::macros::mockable;
use reqwest::blocking::{get, Response};
use reqwest::Error;

fn get_reqwest_error_message(e: Error) -> String {
    match e.status() {
        Some(status_code) => String::from(status_code.as_str()),
        None => String::from("Unknown error"),
    }
}

fn http_get(url: &String) -> Result<Response, String> {
    match get(url) {
        Ok(response) => Ok(response),
        Err(e) => Err(get_reqwest_error_message(e)),
    }
}

#[mockable]
pub fn http_get_text(url: &String) -> Result<String, String> {
    let response = http_get(url)?;
    match response.text() {
        Ok(text) => Ok(text),
        Err(e) => Err(get_reqwest_error_message(e)),
    }
}

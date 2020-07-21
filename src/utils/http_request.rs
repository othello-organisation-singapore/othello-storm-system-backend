use reqwest::blocking::{get, Response};

fn http_get(url: &String) -> Result<Response, String> {
    match get(url) {
        Ok(response) => Ok(response),
        Err(e) => {
            Err(String::from(e.status().as_str()))
        },
    }
}

pub fn http_get_text(url: &String) -> Result<String, String> {
    let response = http_get(url)?;
    response.text()
}

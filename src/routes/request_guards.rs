use rocket::request::{FromRequest, Outcome, Request};

pub struct Token {
    pub jwt: String,
}

impl<'a, 'r> FromRequest<'a, 'r> for Token {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> Outcome<Self, Self::Error> {
        let jwt = request.headers().get_one("X-Authorization").unwrap_or("");
        Outcome::Success(Token {
            jwt: String::from(jwt),
        })
    }
}

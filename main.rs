#![feature(proc_macro_hygiene, decl_macro)]
use rocket::http::RawStr;
#[macro_use] extern crate rocket;
extern crate rocket_cors;
use rocket::Outcome;
use rocket::http::Status;
use rocket::http::Method; // 1.
use rocket::request::{self, Request, FromRequest};

use rocket_cors::{
    AllowedHeaders, AllowedOrigins, Error, // 2.
    Cors, CorsOptions // 3.
};

fn make_cors() -> Cors {
    let allowed_origins = AllowedOrigins::some_exact(&[ // 4.
        "http://localhost:8080",
        "http://127.0.0.1:8080",
        "http://localhost:8000",
        "http://localhost:5500",
        "http://127.0.0.1:5500",
        "http://0.0.0.0:8000",
    ]);

    CorsOptions { // 5.
        allowed_origins,
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(), // 1.
        allowed_headers: AllowedHeaders::some(&[
            "Authorization",
            "Accept",
            "Access-Control-Allow-Origin", // 6.
        ]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors()
    .expect("error while building CORS")
}





struct ApiKey(String);

/// Returns true if `key` is a valid API key string.
fn is_valid(key: &str) -> bool {
    key == "valid_api_key"
}

#[derive(Debug)]
enum ApiKeyError {
    BadCount,
    Missing,
    Invalid,
}

impl<'a, 'r> FromRequest<'a, 'r> for ApiKey {
    type Error = ApiKeyError;

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let keys: Vec<_> = request.headers().get("x-api-key").collect();
       
        match keys.len() {
            0 => Outcome::Failure((Status::BadRequest, ApiKeyError::Missing)),
            1 if is_valid(keys[0]) => Outcome::Success(ApiKey(keys[0].to_string())),
            1 => Outcome::Failure((Status::BadRequest, ApiKeyError::Invalid)),
            _ => Outcome::Failure((Status::BadRequest, ApiKeyError::BadCount)),
        }
    }
}
#[get("/sensitive")]
fn sensitive(key: ApiKey) -> &'static str {
    "Sensitive data."
}

#[get("/world")]
fn world() -> &'static str {
    "Hello, world!"
}
#[get("/hello/<name>")]
fn hello(name: &RawStr) -> String {

    format!("Hello, {}!", name.as_str())
}
fn main() {
    rocket::ignite().mount("/", routes![hello,world,sensitive]).attach(make_cors()).launch();
}
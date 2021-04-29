#![feature(proc_macro_hygiene, decl_macro)]

use rocket::http::Method;
use rocket_cors::{AllowedHeaders, AllowedOrigins};

#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_contrib;

fn main() {
    let cors = rocket_cors::CorsOptions {
        allowed_origins: AllowedOrigins::all(),
        allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&["Authorization", "Accept"]),
        allow_credentials: true,
        ..Default::default()
    }
    .to_cors().unwrap();

    rocket::ignite()
        .mount("/",
            routes![
                test
            ]
        )
        .attach(cors)
        .launch();
}


#[get("/test")]
pub fn test() -> String {
    "Poggers".to_string()
}
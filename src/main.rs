#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use]
extern crate rocket;

use std::path::PathBuf;

use rocket_cors::{AllowedHeaders, AllowedOrigins, Cors, CorsOptions};

const DEFAULT_NAME: &str = ":)";

fn get_name(name: Option<&str>) -> String {
    let name = name.unwrap_or(DEFAULT_NAME).replace('^', "ㅤ");

    println!("Someone is named '{}'", name);

    format!(r#"{{"name": "{}"}}"#, name)
}

#[get("/")]
fn index() -> String {
    get_name(None)
}

#[get("/?<name>")]
fn name_query(name: String) -> String {
    get_name(Some(&name))
}

#[get("/<name..>")]
fn name_path(name: PathBuf) -> String {
    get_name(name.to_str())
}

fn get_cors() -> Cors {
    CorsOptions {
        allowed_origins: AllowedOrigins::All,
        // allowed_methods: vec![Method::Get].into_iter().map(From::from).collect(),
        allowed_headers: AllowedHeaders::some(&[
            // "Authorization",
            // "Accept",
            // "Access-Control-Allow-Origin",
            "Access-Control-Allow-Headers",
            "Origin, X-Requested-With, Content-Type, Accept",
        ]),
        ..Default::default()
    }
    .to_cors()
    .expect("Error creating CORS")
}

fn main() {
    rocket::ignite()
        .mount("/", routes![index, name_query, name_path])
        .attach(get_cors())
        .launch();
}

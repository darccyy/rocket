use std::path::PathBuf;

#[macro_use]
extern crate rocket;

const DEFAULT_NAME: &str = ":)";

fn get_name(name: Option<&str>) -> String {
    let name = name.unwrap_or(DEFAULT_NAME).replace('^', "ㅤ");

    println!("Someone is named '{}'", name);

    format!(r#"{{"name": "{}"}}"#, name)
}

#[get("/?<name>")]
fn default(name: Option<&str>) -> String {
    get_name(name)
}

#[get("/<name..>")]
fn custom(name: PathBuf) -> String {
    get_name(name.to_str())
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![default, custom])
}

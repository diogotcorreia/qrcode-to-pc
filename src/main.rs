use enigo::{Enigo, KeyboardControllable};
use rocket::fs::{FileServer};
use rocket::{post, launch, routes};
use std::env;

#[post("/paste", data = "<input>")]
fn paste_text(input: &str) {
  let mut enigo = Enigo::new();
  enigo.key_sequence(input);
}

#[launch]
fn rocket() -> _ {
    let mut path = env::current_dir().ok().expect("Failed to get current path");
    path.push("src");
    path.push("static");

    println!("Reading static assets from {}", path.display());

    rocket::build()
      .mount("/", routes![paste_text])
      .mount("/", FileServer::from(path))
}

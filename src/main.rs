use enigo::{Enigo, KeyboardControllable};
use rocket::fs::{FileServer, relative};
use rocket::{post, launch, routes};

#[post("/paste", data = "<input>")]
fn paste_text(input: &str) {
  let mut enigo = Enigo::new();
  enigo.key_sequence(input);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
      .mount("/", routes![paste_text])
      .mount("/", FileServer::from(relative!("src/static")))
}

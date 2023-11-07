#[macro_use] extern crate rocket;

#[get("/world")]
fn world() -> &'static str {
  "Hello, world!"
}

#[get("/i2")]
fn index2() -> String {
  "Hello, world!".to_string()
}

#[launch]
fn rocket() -> _ {
  rocket::build()
  .mount("/hello", routes![world, index2])
  .mount("/hi", routes![world, index2])
}

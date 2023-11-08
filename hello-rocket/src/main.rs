use rocket::serde::{Deserialize, Serialize, json::Json};

#[macro_use] extern crate rocket;

#[get("/world")]
fn world() -> &'static str {
  "Hello, world!"
}

#[get("/i2")]
fn index2() -> String {
  "Hello, world!".to_string()
}

#[get("/hello/<name>/<age>/<cool>")]
fn hello(name: &str, age: u8, cool: bool) -> String {
    if cool {
        format!("You're a cool {} year old, {}!", age, name)
    } else {
        format!("{}, we need to talk about your coolness.", name)
    }
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
struct Task {
    description: String,
    complete: bool
}

#[post("/todo", data = "<task>")]
fn todo(task: Json<Task>) -> Json<Task> {
  Json(Task { description: "aaaa".to_string(), complete: true })
 }

#[launch]
fn rocket() -> _ {
  rocket::build()
  .mount("/", routes![world, index2, hello, todo])
}

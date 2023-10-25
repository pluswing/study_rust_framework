use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;

#[get("/")]
async fn hello() -> impl Responder {
  HttpResponse::Ok().body("Hello World!")
}

#[post("/echo")]
async fn echo(req_body: String) -> impl Responder {
  HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
  HttpResponse::Ok().body("Hey there!")
}

#[get("/users/{user_id}/{friend}")]
async fn friend(path: web::Path<(u32, String)>) -> Result<String> {
  let (user_id, friend) = path.into_inner();
  Ok(format!("Wrlcome {}, user_id {}", friend, user_id));
}

#[derive(Deserialize)]
struct Info {
  user_id: u32,
  friend: String,
}

#[get("/users2/{user_id}/{friend}")]
async fn friend2(info: web::Path<Info>) -> Result<String> {
  Ok(format!("Wrlcome {}, user_id {}", info.friend, info.user_id));
}


async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    App::new()
      .service(hello)
      .service(echo)
      .route("/hey", web::get().to(manual_hello))
  }).bind(("127.0.0.1", 8080))?
  .run()
  .await
}

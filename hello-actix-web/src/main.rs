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

#[derive(Deserialize)]
struct Info2 {
    username: String,
}

#[get("/q")]
async fn query(info: web::Query<Info2>) -> String {
    format!("Welcome {}!", info.username)
}

#[derive(Deserialize)]
struct Info3 {
    username: String,
}

#[post("/submit")]
async fn submit(info: web::Json<Info3>) -> Result<String> {
    Ok(format!("Welcome {}!", info.username))
}

// content-type multipart/form-data
#[derive(Deserialize)]
struct FormData {
    username: String,
}

#[post("/form")]
async fn index(form: web::Form<FormData>) -> Result<String> {
    Ok(format!("Welcome {}!", form.username))
}

async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    let json_config = web::JSonConfig::default()
      .limit(4096)
      .error_handle(|err, _req| {
        error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
      });
    App::new()
      .app_data(json_config)
      .service(hello)
      .service(echo)
      .route("/hey", web::get().to(manual_hello))
  }).bind(("127.0.0.1", 8080))?
  .run()
  .await
}

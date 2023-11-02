use std::env;

use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder, Result, body::BoxBody, http::header::ContentType, error};
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};

mod schema;

// type SqlConnection = MysqlConnection;
// type DbPool = r2d2::Pool<r2d2::ConnectionManager<SqlConnection>>;

pub fn establish_connection() -> MysqlConnection {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  MysqlConnection::establish(&database_url)
      .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}



#[derive(Debug, Insertable)]
#[diesel(table_name = self::schema::users)]
struct NewUser<'a> {
  id: &'a u64,
  name: &'a str,
}

fn insert_new_user(
  conn: &mut MysqlConnection,
  user_name: String,
) -> diesel::QueryResult<NewUser> {
  use self::schema::users::dsl::*;

  let new_user = NewUser {
    id: &1,
    name: &user_name,
  };

  diesel::insert_into(users)
    .values(&new_user)
    .execute(conn)
    .expect("Error inserting person");

  let user = users
    // .filter(id.eq(&uid))
    .first::<NewUser>(conn)
    .expect("Error loading person");

  Ok(user)
}

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
  Ok(format!("Wrlcome {}, user_id {}", friend, user_id))
}

#[derive(Deserialize)]
struct Info {
  user_id: u32,
  friend: String,
}

#[get("/users2/{user_id}/{friend}")]
async fn friend2(info: web::Path<Info>) -> Result<String> {
  Ok(format!("Wrlcome {}, user_id {}", info.friend, info.user_id))
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
    let username = info.username.clone();
    let user = web::block(move || {
      let mut conn: MysqlConnection = establish_connection();
      insert_new_user(&mut conn, username);
    })
    .await?
    .map_err(error::ErrorInternalServerError);

    Ok(format!("Welcome {}!", info.username))
}

// content-type application/x-www-form-urlencoded
#[derive(Deserialize)]
struct FormData {
    username: String,
}

#[post("/form")]
async fn form(form: web::Form<FormData>) -> Result<String> {
    Ok(format!("Welcome {}!", form.username))
}

#[derive(Serialize)]
struct MyObj {
  name: String,
}

async fn json_resp() -> Result<impl Responder> {
  let obj: MyObj = MyObj { name: "user".to_string() };
  Ok(web::Json(obj))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  HttpServer::new(|| {
    let json_config = web::JsonConfig::default()
      .limit(4096)
      .error_handler(|err, _req| {
        error::InternalError::from_response(err, HttpResponse::Conflict().finish()).into()
      });
    App::new()
      .app_data(json_config)
      .service(hello)
      .service(echo)
      .route("/hey", web::get().to(manual_hello))
      .service(friend)
      .service(friend2)
      .service(query)
      .service(submit)
      .service(form)
      .route("/json_resp", web::get().to(json_resp))
  }).bind(("0.0.0.0", 8080))?
  .run()
  .await
}

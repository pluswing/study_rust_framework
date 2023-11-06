use std::env;

use actix_web::{get, post, web::{self, Data}, App, HttpResponse, HttpServer, Responder, Result, body::BoxBody, http::header::ContentType, error};
use diesel::mysql::MysqlConnection;
use diesel::r2d2::ConnectionManager;
use diesel::prelude::*;
use dotenvy::dotenv;
use serde::{Deserialize, Serialize};

mod schema;

type SqlConnection = MysqlConnection;
type DbPool = r2d2::Pool<ConnectionManager<SqlConnection>>;

pub fn establish_connection() -> DbPool {
  dotenv().ok();

  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let manager = ConnectionManager::<SqlConnection>::new(database_url);
    let pool: DbPool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create pool.");
  pool
}


#[derive(Debug, Insertable, Queryable)]
#[diesel(table_name = self::schema::users)]
struct NewUser {
  id: u64,
  name: String,
}

fn insert_new_user(
  conn: &mut SqlConnection,
  user_name: String,
) -> diesel::QueryResult<NewUser> {
  use self::schema::users::dsl::*;

  let new_user = NewUser {
    id: 1,
    name: user_name,
  };

  diesel::insert_into(users)
    .values(&new_user)
    .execute(conn)
    .expect("Error inserting person");

  let user = users
    .filter(id.eq(1))
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
async fn submit(pool: web::Data<DbPool>, info: web::Json<Info3>) -> Result<String> {
    let user = web::block(move || {
      let mut conn = pool.get().unwrap();
      insert_new_user(&mut conn, info.username.clone())
    })
    .await.unwrap();

    Ok(format!("Welcome {:?}!", user.unwrap()))
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
    let pool = establish_connection();
    App::new()
      .app_data(json_config)
      .app_data(Data::new(pool.clone()))
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

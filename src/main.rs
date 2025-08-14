use actix_web::{
  get,
  post,
  web,
  App,
  HttpResponse,
  HttpServer,
  Responder,
};
use actix_web_lab::sse::Sse;
use std::time::Duration;
use tokio::sync::mpsc;

struct CUID2(String);

struct Message {
  to: String,
  at: String,
  kind: String,
  body: String,
}

#[post("/message")]
async fn message(state: web::Data<AppState>) -> impl Responder {
  let mut pool = state.pool.lock();
  if let Some(tx) = pool.get(todo!()) {
    tx.send(todo!());
  }
  HttpResponse::Ok();
}

#[get("/subscribe")]
async fn subscribe(state: web::Data<AppState>) -> impl Responder {
  // validate session token
  //TODO

  let (tx, rx) = mpsc::channel(10);

  // register channel
  {
    let mut pool = state.pool.lock();
    pool.insert("asdf", tx);
  }

  Sse::from_infallible_receiver(rx)
  .with_retry_duration(Duration::from_secs(10))
}

struct AppState {
  db: PgPool,
  pool: Mutex<HashMap<CUID2, Sender>>,
}

#[actix_web::main]
async fn main() -> Result<(), impl Error> {
  let state = web::Data::new(state::AppState::new("")?);

  HttpServer::new(move || {
    App::new()
    .app_data(state.clone())
    .service(message)
    .service(subscribe)
  })
  .bind(("127.0.0.1", 8080))?
  .run()
  .await
}

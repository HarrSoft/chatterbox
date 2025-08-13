use actix_web::{get, post, web, App, HttpServer, Responder, Response};
use actix_web_lab::sse: Sse;
use parking_lot::Mutex;
use std::collections::HashMap;
use tokio::sync::mpsc;

enum CUID2(String);

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
  Response::Ok();
}

#[get("/subscribe")]
async fn subscribe(state: web::Data<AppState>) -> impl Responder {
  // validate session token
  todo!();

  let (tx, rx) = mpsc::channel(10);

  // register channel
  {
    let mut pool = data.pool.lock();
    pool.insert(todo!(), )
  }

  Sse::from_infallible_receiver(rx)
  .with_retry_duration(Duration::from_secs(10))
}

struct AppState {
  pool: Mutex<HashMap<CUID2, Sender>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
  let state = web::Data::new(AppState {
    pool: Mutex::new(HashMap::new()),
  });

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

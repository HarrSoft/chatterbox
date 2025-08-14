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
use crate::{
  backlog::fetch_backlog,
  session::fetch_session,
  state::AppState,
};
use std::time::Duration;
use tokio::sync::mpsc;

#[actix_web::main]
async fn main() -> Result<(), dyn Error> {
  let env = config::Env::nab();

  let state = web::Data::new(AppState::new(env.database_url)?);

  HttpServer::new(move || {
    App::new()
    .app_data(state.clone())
    .service(message)
    .service(subscribe)
  })
  .bind(("0.0.0.0", 8080))?
  .run()
  .await
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
async fn subscribe(
  state: web::Data<AppState>,
  ts: web::Query<&str>,
  req: HttpRequest,
) -> impl Responder {
  // get session cookie
  let Some(cookie) = req.cookie("session") else {
    return HttpResponse::Unauthorized();
  };

  // get session from db
  let session = match fetch_session(&state, cookie.value()) {
    Ok(session) => session,
    Err(e) => {
      error!("Failed to fetch session: {}", e);
      return HttpResponse::Unauthorized();
    },
  };

  let (tx, rx) = mpsc::channel(10);

  // register channel
  {
    let mut pool = state.pool.lock();
    pool.insert(session.user_id, tx.clone());
  }

  // backfill old messages
  tokio::spawn(async move || {
    let backlog = fetch_backlog(session.user_id, ts).await;
    match backlog {
      Ok(bl) => for message in bl {
        tx.send(message);
      },
      Err(e) => {
        warn!("Failed to fetch backlog: {:?}", e);
      },
    }
  });

  // stream incoming messages
  Sse::from_infallible_receiver(rx)
    .with_retry_duration(Duration::from_secs(10))
}

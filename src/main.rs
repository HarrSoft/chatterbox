use actix_web::{
  get,
  post,
  web,
  App,
  HttpRequest,
  HttpResponse,
  HttpServer,
  Responder,
};
use actix_web_lab::sse;
use chatterbox::{
  backlog::fetch_backlog,
  config::Env,
  database,
  session::fetch_session,
  state::AppState,
  CUID2,
  Message,
};
use log::*;
use serde::Deserialize;
use std::time::Duration;
use time::OffsetDateTime;
use tokio::sync::mpsc;

#[actix_web::main]
async fn main() {
  let env = Env::nab();

  let state = web::Data::new(AppState::new(env.database_url).await.unwrap());

  // initialize database if needed
  database::postgres::init(&state.db).await.unwrap();

  HttpServer::new(move || {
    App::new()
    .app_data(state.clone())
    .service(message)
    .service(subscribe)
  })
  .bind(env.bind_address).unwrap()
  .run()
  .await;
}

#[derive(Deserialize)]
struct PostMessageArgs {
  to: CUID2,
  kind: String,
  body: String,
}

#[post("/message")]
async fn message(
  state: web::Data<AppState>,
  args: web::Json<PostMessageArgs>,
) -> impl Responder {
  // assemble message from request body
  let PostMessageArgs { to, kind, body } = args.0;
  let message = Message {
    id: cuid2::create_id(),
    to,
    kind,
    body,
    created_at: OffsetDateTime::now_utc(),
  };

  // transmit message
  {
    let client_streams = state.client_streams.read();
    if let Some(tx) = client_streams.get(&message.to) {
      tx.send(message.clone());
    }
  }

  // store message
  sqlx::query(r#"
    INSERT INTO "Chatterbox" ("id", "to", "kind", "body", "createdAt")
    VALUES ($1, $2, $3, $4, $5);
  "#)
    .bind(message.id)
    .bind(message.to)
    .bind(message.kind)
    .bind(message.body)
    .bind(message.created_at)
    .execute(&state.db)
    .await;

  HttpResponse::Ok().finish()
}

#[get("/subscribe")]
async fn subscribe(
  state: web::Data<AppState>,
  ts: web::Query<&str>,
  req: HttpRequest,
) -> impl Responder {
  // get session cookie
  let Some(cookie) = req.cookie("session") else {
    return HttpResponse::Unauthorized().finish();
  };

  // get session from db
  let session = match fetch_session(&state, cookie.value()).await {
    Ok(session) => session,
    Err(e) => {
      error!("Failed to fetch session: {}", e);
      return HttpResponse::Unauthorized().finish();
    },
  };

  let (tx, rx) = mpsc::channel(10);

  // register channel
  {
    let mut streams = state.client_streams.write();
    streams.insert(session.user_id, tx.clone());
  }

  // backfill old messages
  tokio::spawn(async move || {
    let backlog = fetch_backlog(&session.user_id, ts).await;
    match backlog {
      Ok(blz) => for message in blz {
        let json = sse::Data::new_json(message);
        tx.send(Event::Data(json));
      },
      Err(e) => {
        warn!("Failed to fetch backlog: {:?}", e);
      },
    }
  });

  // stream incoming messages
  sse::Sse::from_infallible_receiver(rx)
    .with_retry_duration(Duration::from_secs(10))
}

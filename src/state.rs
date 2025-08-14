use crate::{CUID2, Message};
use parking_lot::RwLock;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::collections::HashMap;
use tokio::sync::mpsc;

pub struct AppState {
  pub client_streams: RwLock<HashMap<CUID2, mpsc::Sender<Message>>>,
  pub db: PgPool,
}

impl AppState {
  pub async fn new(db: impl AsRef<str>) -> Result<Self, sqlx::Error> {
    Ok(Self {
      client_streams: RwLock::new(HashMap::new()),
      db: PgPoolOptions::new().connect(db.as_ref()).await?,
    })
  }
}

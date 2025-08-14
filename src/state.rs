use crate::CUID2;
use parking_lot::RwLock;
use sqlx::postgres::PgPoolOptions;
use tokio::sync::mpsc;

pub struct AppState {
  pub client_streams: RwLock<HashMap<CUID2, mpsc::Sender<Message>>>,
  pub db: PgPool,
}

impl AppState {
  pub async fn new(db: &str) -> Result<Self, sqlx::Error> {
    Self {
      client_streams: RwLock::new(HashMap::new()),
      db: PgPoolOptions::new().connect(db).await?,
    }
  }
}

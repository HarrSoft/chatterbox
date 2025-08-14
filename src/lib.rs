pub mod backlog;
pub mod config;
pub mod database;
pub mod session;
pub mod state;

use serde::Serialize;

pub type CUID2 = String;

#[derive(Clone, Serialize, sqlx::FromRow)]
#[sqlx(rename_all = "camelCase")]
pub struct Message {
  pub id: CUID2,
  pub to: CUID2,
  pub kind: String,
  pub body: String,
  pub created_at: time::OffsetDateTime,
}

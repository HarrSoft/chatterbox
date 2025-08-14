mod backlog;
mod config;
mod session;
mod state;

#[derive(Hash)]
pub struct CUID2(pub String);

pub struct Message {
  pub id: CUID2,
  pub to: CUID2,
  pub at: String,
  pub kind: String,
  pub body: String,
}

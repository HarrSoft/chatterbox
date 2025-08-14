use std::env;

pub struct Env {
  pub database_url: String,
}

impl Env {
  pub fn nab() -> Self {
    Self {
      database_url: env::var("DATABASE_URL")
        .expect("Env variable DATABASE_URL must be set!"),
    }
  }
}

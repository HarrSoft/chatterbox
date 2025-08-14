use std::env;

pub enum Database {
  Postgres,
}

pub struct Env {
  pub database_url: String,
  pub database: Database,
}

impl Env {
  pub fn nab() -> Self {
    let database_url = env::var("DATABASE_URL")
      .expect("Env variable DATABASE_URL must be set!");

    Self {
      database: match database_url.split(':').next() {
        Some("postgres") => Database::Postgres,
        Some(_) |
        None => panic!("Could not determine database kind from DATABASE_URL"),
      },
      database_url,
    }
  }
}

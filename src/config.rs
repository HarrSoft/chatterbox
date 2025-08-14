use std::env;

pub enum Database {
  Postgres,
}

pub struct Env {
  pub bind_address: String,
  pub database: Database,
  pub database_url: String,
}

impl Env {
  pub fn nab() -> Self {
    let database_url = env::var("DATABASE_URL")
      .expect("Env variable DATABASE_URL must be set!");

    Self {
      bind_address: env::var("BIND_ADDRESS")
        .unwrap_or_else(|_| String::from("0.0.0.0:8080")),
      database: match database_url.split(':').next() {
        Some("postgres") => Database::Postgres,
        Some(_) |
        None => panic!("Could not determine database kind from DATABASE_URL"),
      },
      database_url,
    }
  }
}

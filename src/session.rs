use crate::state::AppState;
use sha2::{Digest, Sha256};
use std::fmt::Write;

#[derive(sqlx::FromRow)]
#[sqlx(rename_all = "camelCase")]
pub struct Session {
  pub id: String,
  pub user_id: String,
  pub token: String,
  pub expires: String,
}

pub async fn fetch_session(
  state: &AppState,
  token: impl AsRef<[u8]>,
) -> Result<Session, sqlx::Error> {
  sqlx::query_as(r#"
    SELECT "id", "userId", "expires"
    FROM "Session"
    WHERE "token" = $1;
  "#)
    .bind(&encode_token(token))
    .fetch_one(&state.db)
    .await
}

fn encode_token(data: impl AsRef<[u8]>) -> String {
  let digest = Sha256::digest(data);
  let mut buffer = String::with_capacity(digest.len() * 2);
  for byte in digest {
    write!(&mut buffer, "{:x?}", byte).unwrap();
  }
  buffer
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn token_encodes_correctly() {
    let token = "wab7mlxt2gnciiwx47f3sitloaint6ko";
    let expected_digest = "fc6d9243d26179fdee160fcca82a0b8b9bb0e5605af54430b0848dd1879ac6ff";
    let actual_digest = encode_token(token);

    assert_eq!(expected_digest, actual_digest);
  }
}

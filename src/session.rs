use crate::state::AppState;
use sha2::Sha256;

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
  let token = encode_token(token);

  let (
    id,
    user_id,
    expires,
  ): (
    String,
    String,
    String,
  ) = sqlx::query_as("\
    SELECT id, userId, expires \
    FROM Session \
    WHERE token = $1;\
  ")
    .bind(token)
    .fetch_one(&state.db)
    .await?;

  Ok(Session { id, user_id, expires, token });
}

fn encode_token(data: impl AsRef<[u8]>) -> String {
  let digest = Sha256::digest(data);
  let mut buffer = String::with_capacity(digest.len() * 2);
  for byte in digest {
    write!(&buffer, "{:x?}", byte);
  }
  buffer
}

#[cfg(test)]
mod test {
  use sha2::Sha256;
  use super::*;

  #[test]
  fn token_encodes_correctly() {
    //TODO
  }
}

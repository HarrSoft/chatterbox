use crate::{
  state::AppState,
  Message,
};
use time::OffsetDateTime;

pub async fn fetch_backlog(
  state: &AppState,
  user_id: impl AsRef<str>,
  timestamp: Option<OffsetDateTime>,
) -> Result<Vec<Message>, sqlx::Error> {
  let rows: Vec<Message> = if let Some(ts) = timestamp {
    sqlx::query_as(r#"
      SELECT *
      FROM ChatterboxMessage
      WHERE userId = $1
        AND createdAt > $2
      ORDER BY createdAt DESC
      LIMIT 100;
    "#)
      .bind::<&str>(user_id.as_ref())
      .bind(ts)
      .fetch_all(&state.db)
      .await?
  } else {
    sqlx::query_as(r#"
      SELECT *
      FROM ChatterboxMessage
      WHERE userId = $1
      ORDER BY createdAt DESC
      LIMIT 100;
    "#)
      .bind::<&str>(user_id.as_ref())
      .fetch_all(&state.db)
      .await?
  };

  Ok(rows)
}

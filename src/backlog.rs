use crate::state::AppState;

pub async fn fetch_backlog(
  state: &AppState,
  user_id: impl AsRef<str>,
  timestamp: Option<impl AsRef<str>>,
) -> Result<Vec<Message>, sqlx::Error> {
  let messages = if let Some(ts) = timestamp {
    sqlx::query_as("\
      SELECT * \
      FROM Chatterbox \
      WHERE userId = $1 \
        AND createdAt > $2 \
      ORDER BY createdAt ASC;\
    ")
      .bind(user_id.as_ref())
      .bind(ts.as_ref())
      .fetch_all(&state.db)
      .await?
  } else {
    sqlx::query_as("\
      SELECT * \
      FROM Chatterbox \
      WHERE userId = $1\
      ORDER BY createdAt ASC;\
    ")
      .bind(user_id.as_ref())
      .fetch_all(&state.db)
      .await?
  };

  todo!();
}

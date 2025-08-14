use sqlx::{
  Error,
  PgPool,
  postgres::PgQueryResult,
  query,
};

pub async fn init(pool: &PgPool) -> Result<PgQueryResult, Error> {
  query(r#"
    CREATE TABLE IF NOT EXISTS "Chatterbox" (
      "id"        CHAR(24)     NOT NULL,
      "to"        CHAR(24)     NOT NULL,
      "kind"      VARCHAR(10)  NOT NULL,
      "body"      TEXT         NOT NULL,
      "createdAt" TIMESTAMP(3) NOT NULL DEFAULT CURRENT_TIMESTAMP,

      CONSTRAINT "Chatterbox_pkey" PRIMARY KEY ("id")
    );
  "#)
    .execute(pool)
    .await
}

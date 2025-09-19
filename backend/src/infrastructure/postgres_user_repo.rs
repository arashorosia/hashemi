use crate::domain::repository::UserRepository;
use crate::domain::user::User;
use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;

#[derive(Clone)]
pub struct PostgresUserRepo {
    pool: PgPool,
}

impl PostgresUserRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl UserRepository for PostgresUserRepo {
    async fn find_by_email(&self, email: &str) -> Result<Option<User>> {
        let rec = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, created_at, updated_at FROM users WHERE email = $1",
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await?;
        Ok(rec)
    }

    async fn save(&self, user: &User) -> Result<()> {
        sqlx::query(
            "INSERT INTO users (id, email, password_hash, created_at, updated_at) VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(user.id)
        .bind(&user.email)
        .bind(&user.password_hash)
        .bind(user.created_at)
        .bind(user.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>> {
        let rec = sqlx::query_as::<_, User>(
            "SELECT id, email, password_hash, created_at, updated_at FROM users WHERE id = $1",
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await?;
        Ok(rec)
    }
}

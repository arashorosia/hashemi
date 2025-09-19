use crate::domain::user::User;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait UserRepository {
    async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<User>>;
    async fn save(&self, user: &User) -> anyhow::Result<()>;
    async fn find_by_id(&self, id: Uuid) -> anyhow::Result<Option<User>>;
}

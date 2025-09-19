use crate::domain::repository::UserRepository;
use crate::domain::user::User;
use bcrypt::verify;
use jsonwebtoken::{encode, EncodingKey, Header};
use chrono::{Utc, Duration};
use uuid::Uuid;
use anyhow::{Result, anyhow};

#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct Claims {
    sub: Uuid,
    email: String,
    exp: i64,
    iat: i64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::user::User;
    use crate::domain::repository::UserRepository;
    use async_trait::async_trait;
    use chrono::Utc;
    use uuid::Uuid;
    use bcrypt::{hash, DEFAULT_COST};
    use jsonwebtoken::{decode, DecodingKey, Validation};

    #[derive(Clone)]
    struct DummyRepo {
        user: Option<User>,
    }

    #[async_trait]
    impl UserRepository for DummyRepo {
        async fn find_by_email(&self, email: &str) -> anyhow::Result<Option<User>> {
            Ok(self.user.clone().filter(|u| u.email == email))
        }
        async fn save(&self, _user: &User) -> anyhow::Result<()> { Ok(()) }
        async fn find_by_id(&self, _id: Uuid) -> anyhow::Result<Option<User>> { Ok(None) }
    }

    #[tokio::test]
    async fn login_success_returns_valid_jwt() {
        // prepare a dummy user with known password
        let password = "secret";
        let hash_pw = hash(password, DEFAULT_COST).unwrap();
        let user = User {
            id: Uuid::new_v4(),
            email: "test@example.com".into(),
            password_hash: hash_pw.clone(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };
        let repo = DummyRepo { user: Some(user.clone()) };
        let service = UserService::new(repo, "mysecret".into(), 3600);
        let token = service.login(&user.email, password).await.expect("login failed");
        // decode token
        let decoded = decode::<Claims>(&token, &DecodingKey::from_secret("mysecret".as_ref()), &Validation::default()).unwrap();
        assert_eq!(decoded.claims.email, user.email);
        assert_eq!(decoded.claims.sub, user.id);
    }

    #[tokio::test]
    async fn login_failure_wrong_password() {
        let password = "secret";
        let hash_pw = hash(password, DEFAULT_COST).unwrap();
        let user = User { id: Uuid::new_v4(), email: "test@example.com".into(), password_hash: hash_pw, created_at: Utc::now(), updated_at: Utc::now() };
        let repo = DummyRepo { user: Some(user) };
        let service = UserService::new(repo, "mysecret".into(), 3600);
        let result = service.login("test@example.com", "wrong").await;
        assert!(result.is_err());
    }
}

#[derive(Clone)]
pub struct UserService<R: UserRepository + Clone> {
    repo: R,
    jwt_secret: String,
    jwt_exp: i64,
}

impl<R: UserRepository + Clone> UserService<R> {
    pub fn new(repo: R, jwt_secret: String, jwt_exp: i64) -> Self {
        Self { repo, jwt_secret, jwt_exp }
    }

    pub async fn login(&self, email: &str, password: &str) -> Result<String> {
        let user_opt = self.repo.find_by_email(email).await?;
        let user = user_opt.ok_or_else(|| anyhow!("Invalid email or password"))?;

        if !verify(password, &user.password_hash)? {
            return Err(anyhow!("Invalid email or password"));
        }

        let now = Utc::now();
        let exp = now + Duration::seconds(self.jwt_exp);
        let claims = Claims {
            sub: user.id,
            email: user.email.clone(),
            exp: exp.timestamp(),
            iat: now.timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_ref()),
        )?;

        Ok(token)
    }
}

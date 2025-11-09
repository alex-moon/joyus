use sqlx::PgPool;
use uuid::Uuid;

#[derive(Clone, Debug)]
pub struct UserSummary {
    pub id: Uuid,
    pub name: String,
}

#[derive(Clone)]
pub struct UserService {
    pool: PgPool,
}

impl UserService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    /// Ensures there is exactly one user in the DB and returns it.
    pub async fn ensure_single_user(&self) -> Result<UserSummary, String> {
        // Check if a user already exists
        let existing = sqlx::query!(
            r#"SELECT id, name FROM users LIMIT 1"#
        )
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        if let Some(row) = existing {
            return Ok(UserSummary {
                id: row.id,
                name: row.name,
            });
        }

        // Otherwise, insert the one and only user
        let id = Uuid::new_v4();
        let name = "Guest".to_string();

        sqlx::query!(
            r#"INSERT INTO users (id, name) VALUES ($1, $2)"#,
            id,
            name
        )
            .execute(&self.pool)
            .await
            .map_err(|e| e.to_string())?;

        Ok(UserSummary { id, name })
    }

    /// Public accessor — always returns the single user
    pub async fn summary(&self) -> UserSummary {
        self.ensure_single_user()
            .await
            .expect("failed to ensure default user")
    }
}

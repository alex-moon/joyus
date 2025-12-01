use axum::extract::State;
use axum::http::StatusCode;
use axum::Json;
use serde::Deserialize;
use sqlx::{PgPool, Row};
use tower_sessions::Session;
use uuid::Uuid;
use crate::service::state::AppState;

const APP_USER_ID_KEY: &str = "app_user_id";

#[derive(Deserialize)]
pub struct Location {
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
}

#[derive(Clone, Debug)]
pub struct User {
    pub id: Uuid,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[derive(Clone)]
pub struct UserService {
    pool: PgPool,
}

impl UserService {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    // This method handles the session-to-DB mapping completely.
    pub async fn get_or_create_session_user(&self, session: Session) -> Result<User, String> {
        // 1. Try to load the application's User ID from the session store
        let user_id_option: Option<Uuid> = session.get(APP_USER_ID_KEY).await
            .map_err(|e| format!("Session read error: {}", e))?;

        let final_user = if let Some(id) = user_id_option {
            // A. User ID found: Load user from the database.
            self.get_by_id(&id).await?
        } else {
            // B. New Session: Create a new anonymous user.
            let new_user = self.create_anonymous_user().await?;

            // C. Store the new user's ID in the tower-session object.
            // This implicitly triggers the session middleware to generate a Session ID
            // and send the Set-Cookie header in the response.
            session.insert(APP_USER_ID_KEY, new_user.id).await
                .map_err(|e| format!("Session write error: {}", e))?;

            new_user
        };

        Ok(final_user)
    }

    pub async fn create_anonymous_user(&self) -> Result<User, String> {
        let row = sqlx::query(r#"
            INSERT INTO users DEFAULT VALUES
            RETURNING id
        "#)
        .fetch_one(&self.pool)
        .await
        .map_err(|e| e.to_string())?;

        Ok(User {
            id: row.get::<Uuid, _>("id"),
            latitude: None,
            longitude: None,
        })
    }

    pub async fn update_location(&self, id: &Uuid, longitude: f64, latitude: f64) -> Result<(), String> {
        tracing::debug!("attempting to update location {} {} {}", id, longitude, latitude);
        sqlx::query(
            r#"UPDATE users SET point = ST_MakePoint($2, $3) WHERE id = $1"#
        )
        .bind(id)
        .bind(longitude)
        .bind(latitude)
        .execute(&self.pool)
        .await
        .map_err(|e| e.to_string())?;
        Ok(())
    }

    pub async fn get_by_id(&self, id: &Uuid) -> Result<User, String> {
        let result = sqlx::query_as!(
            User,
            r#"
                SELECT
                    id,
                    ST_X(point::geometry) AS longitude,
                    ST_Y(point::geometry) AS latitude
                FROM users WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool) // 2. Use fetch_optional to get 0 or 1 row.
        .await
        .map_err(|e| e.to_string())?
        .ok_or_else(|| "User not found".to_string())?;
        Ok(result)
    }
}

pub async fn update_user(
    State(state): State<AppState>,
    session: Session,
    Json(form): Json<Location>,
) -> Result<StatusCode, String> {
    let user = state.users.get_or_create_session_user(session).await?;

    if let (Some(lon), Some(lat)) = (form.longitude, form.latitude) {
        // persist the last known location for this user
        if let Err(e) = state.users.update_location(&user.id, lat, lon).await {
            tracing::warn!(error = %e, "failed to update user location");
        }
    }
    // 3. Return 204 No Content on success
    Ok(StatusCode::NO_CONTENT)
}

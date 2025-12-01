use sqlx::{PgPool, Row};
use time::OffsetDateTime;
use uuid::Uuid;
use serde::Serialize;

#[derive(Clone, Debug, Serialize)]
pub struct Joy {
    pub id: Uuid,
    pub user_id: Uuid,
    pub longitude: Option<f64>,
    pub latitude: Option<f64>,
    pub frustration: Option<String>,
    pub context: Option<String>,
    pub joy: String,
    #[serde(with = "time::serde::iso8601")]
    pub created: OffsetDateTime,
    pub distance: Option<f64>,
}

#[derive(Clone)]
pub struct JoyService {
    db: PgPool,
}

impl JoyService {
    pub fn new(db: PgPool) -> Self {
        Self { db }
    }

    fn validate(&self, frustration: &str, context: &str, joy: &str) -> Result<(), String> {
        let check = |name: &str, value: &str| -> Result<(), String> {
            let t = value.trim();
            if t.is_empty() {
                return Err(format!("{} cannot be empty", name));
            }
            Ok(())
        };
        check("frustration", frustration)?;
        check("context", context)?;
        check("joy", joy)?;
        Ok(())
    }

    pub async fn list_for_user(&self, user_id: &Uuid) -> Result<Vec<Joy>, String> {
        let row = sqlx::query(r#"
            SELECT point IS NOT NULL AS has_point
            FROM users WHERE id = $1
        "#)
            .bind(user_id)
            .fetch_one(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        let has_point = row.get::<bool, _>("has_point");

        let rows = if has_point {
            sqlx::query(
                r#"
                    SELECT
                        j.id,
                        j.user_id,
                        j.created,
                        ST_X(j.point::geometry) AS longitude,
                        ST_Y(j.point::geometry) AS latitude,
                        j.joy,
                        ST_DISTANCE(j.point, u.point) AS distance
                    FROM joys j
                    JOIN users u ON u.id = $1
                    WHERE u.point IS NOT NULL
                        AND j.point IS NOT NULL
                        AND ST_DWithin(j.point, u.point, 1000000)
                    ORDER BY ST_Distance(j.point, u.point) ASC
                "#,
            )
                .bind(user_id)
                .fetch_all(&self.db)
                .await
                .map_err(|e| e.to_string())?
        } else {
            sqlx::query(
                r#"
                    SELECT
                        j.id,
                        j.user_id,
                        j.created,
                        ST_X(j.point::geometry) AS longitude,
                        ST_Y(j.point::geometry) AS latitude,
                        j.joy,
                        NULL AS distance
                    FROM joys j
                    JOIN users u ON u.id = $1
                    WHERE u.point IS NULL
                        AND j.created >= (NOW() - INTERVAL '24 hour')
                    ORDER BY j.created DESC
                "#,
            )
                .bind(user_id)
                .fetch_all(&self.db)
                .await
                .map_err(|e| e.to_string())?
        };

        let joys = rows
            .into_iter()
            .map(|row| Joy {
                id: row.get::<Uuid, _>("id"),
                user_id: row.get::<Uuid, _>("user_id"),
                created: row.get::<OffsetDateTime, _>("created"),
                longitude: row.get::<Option<f64>, _>("longitude"),
                latitude: row.get::<Option<f64>, _>("latitude"),
                frustration: None,
                context: None,
                joy: row.get::<String, _>("joy"),
                distance: row.get::<Option<f64>, _>("distance"),
            })
            .collect();

        Ok(joys)
    }

    pub async fn create(
        &self,
        user_id: &Uuid,
        frustration: String,
        context: String,
        joy: String,
    ) -> Result<Joy, String> {
        self.validate(&frustration, &context, &joy)?;

        let row = sqlx::query(r#"
            INSERT INTO joys (user_id, point, frustration, context, joy, created)
            SELECT id, point, $1, $2, $3, NOW()
            FROM users WHERE id = $4
            RETURNING id, user_id, frustration, context, joy, created,
              ST_X(point::geometry) AS longitude,
              ST_Y(point::geometry) AS latitude
        "#)
            .bind(frustration.trim())
            .bind(context.trim())
            .bind(joy.trim())
            .bind(user_id)
            .fetch_one(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(Joy {
            id: row.get::<Uuid, _>("id"),
            user_id: row.get::<Uuid, _>("user_id"),
            created: row.get::<OffsetDateTime, _>("created"),
            longitude: row.get::<Option<f64>, _>("longitude"),
            latitude: row.get::<Option<f64>, _>("latitude"),
            frustration: row.get::<Option<String>, _>("frustration"),
            context: row.get::<Option<String>, _>("context"),
            joy: row.get::<String, _>("joy"),
            distance: Some(0f64),
        })
    }

    pub async fn get_for_user(&self, id: Uuid, user_id: Uuid) -> Result<Option<Joy>, String> {
        let row = sqlx::query(
            r#"
                SELECT
                    id,
                    user_id,
                    created,
                    ST_X(point::geometry) AS longitude,
                    ST_Y(point::geometry) AS latitude,
                    joy,
                    ST_Distance(
                        point,
                        (SELECT point FROM users WHERE id = $2)
                    ) AS distance
                FROM joys
                WHERE id = $1
            "#,
        )
            .bind(id)
            .bind(user_id)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(row.map(|row| Joy {
            id: row.get::<Uuid, _>("id"),
            user_id: row.get::<Uuid, _>("user_id"),
            longitude: row.get::<Option<f64>, _>("longitude"),
            latitude: row.get::<Option<f64>, _>("latitude"),
            frustration: None,
            context: None,
            joy: row.get::<String, _>("joy"),
            created: row.get::<OffsetDateTime, _>("created"),
            distance: row.get::<Option<f64>, _>("distance"),
        }))
    }
}

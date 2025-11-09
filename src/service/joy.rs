use sqlx::{PgPool, Row};
use uuid::Uuid;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Debug)]
pub struct Point {
    pub lon: f64,
    pub lat: f64,
}

impl Point {
    pub fn new(lon: f64, lat: f64) -> Result<Self, String> {
        if !(-180.0..=180.0).contains(&lon) {
            return Err("longitude must be between -180 and 180".into());
        }
        if !(-90.0..=90.0).contains(&lat) {
            return Err("latitude must be between -90 and 90".into());
        }
        Ok(Self { lon, lat })
    }
}

#[derive(Clone, Debug)]
pub struct Joy {
    pub id: Uuid,
    pub user_id: Uuid,
    pub point: Option<Point>,
    pub frustration: String,
    pub context: String,
    pub joy: String,
    pub created: i64, // unix millis
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
        let rows = sqlx::query(
            r#"
                SELECT id, user_id,
                       ST_X(point::geometry) AS lon,
                       ST_Y(point::geometry) AS lat,
                       frustration, context, joy, created
                FROM joys
                WHERE user_id = $1
                ORDER BY created DESC
            "#,
        )
            .bind(user_id)
            .fetch_all(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        let joys = rows
            .into_iter()
            .map(|row| Joy {
                id: row.get::<Uuid, _>("id"),
                user_id: row.get::<Uuid, _>("user_id"),
                point: match (
                    row.try_get::<f64, _>("lon"),
                    row.try_get::<f64, _>("lat"),
                ) {
                    (Ok(lon), Ok(lat)) => Some(Point { lon, lat }),
                    _ => None,
                },
                frustration: row.get::<String, _>("frustration"),
                context: row.get::<String, _>("context"),
                joy: row.get::<String, _>("joy"),
                created: row.get::<i64, _>("created"),
            })
            .collect();

        Ok(joys)
    }

    pub async fn create(
        &self,
        user_id: &Uuid,
        point: Option<Point>,
        frustration: String,
        context: String,
        joy: String,
    ) -> Result<Joy, String> {
        self.validate(&frustration, &context, &joy)?;

        let created = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as i64;

        let id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO joys (id, user_id, point, frustration, context, joy, created)
            VALUES ($1, $2,
                CASE WHEN $3 IS NULL THEN NULL
                     ELSE ST_SetSRID(ST_MakePoint($4, $5), 4326)::GEOGRAPHY
                END,
                $6, $7, $8, $9
            )
            "#,
        )
            .bind(id)
            .bind(user_id)
            .bind(point.is_some())
            .bind(point.as_ref().map(|p| p.lon))
            .bind(point.as_ref().map(|p| p.lat))
            .bind(frustration.trim())
            .bind(context.trim())
            .bind(joy.trim())
            .bind(created)
            .execute(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(Joy {
            id,
            user_id: *user_id,
            point,
            frustration: frustration.trim().to_string(),
            context: context.trim().to_string(),
            joy: joy.trim().to_string(),
            created,
        })
    }

    pub async fn get(&self, id: Uuid) -> Result<Option<Joy>, String> {
        let row = sqlx::query(
            r#"
                SELECT id, user_id,
                       ST_X(point::geometry) AS lon,
                       ST_Y(point::geometry) AS lat,
                       frustration, context, joy, created
                FROM joys
                WHERE id = $1
            "#,
        )
            .bind(id)
            .fetch_optional(&self.db)
            .await
            .map_err(|e| e.to_string())?;

        Ok(row.map(|row| Joy {
            id: row.get::<Uuid, _>("id"),
            user_id: row.get::<Uuid, _>("user_id"),
            point: match (
                row.try_get::<f64, _>("lon"),
                row.try_get::<f64, _>("lat"),
            ) {
                (Ok(lon), Ok(lat)) => Some(Point { lon, lat }),
                _ => None,
            },
            frustration: row.get::<String, _>("frustration"),
            context: row.get::<String, _>("context"),
            joy: row.get::<String, _>("joy"),
            created: row.get::<i64, _>("created"),
        }))
    }
}

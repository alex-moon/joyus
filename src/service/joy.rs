use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

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
}

#[derive(Clone, Default)]
pub struct JoyService {
    // very simple in-memory store keyed by user id
    inner: Arc<RwLock<HashMap<Uuid, Vec<Joy>>>>,
}

impl JoyService {
    pub fn new() -> Self {
        Self { inner: Arc::new(RwLock::new(HashMap::new())) }
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

    pub async fn list_for_user(&self, user_id: &Uuid) -> Vec<Joy> {
        let map = self.inner.read().await;
        map.get(user_id).cloned().unwrap_or_default()
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

        let joy_item = Joy {
            id: Uuid::new_v4(),
            user_id: *user_id,
            point,
            frustration: frustration.trim().to_string(),
            context: context.trim().to_string(),
            joy: joy.trim().to_string(),
        };

        let mut map = self.inner.write().await;
        let entry = map.entry(*user_id).or_default();
        entry.push(joy_item.clone());
        Ok(joy_item)
    }
}

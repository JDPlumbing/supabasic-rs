use crate::error::Result;
use crate::Supabase;
use serde::Deserialize;
use serde_json::json;
use uuid::Uuid;

/// The shape of an entity row (simplified for now)
#[derive(Debug, Deserialize)]
pub struct Entity {
    pub id: Uuid,
    pub name: Option<String>, // assuming you have a `name` field
}

impl Supabase {
    /// Create a new entity and return its ID
    pub async fn create_entity(&self, name: &str) -> Result<Uuid> {
        let payload = json!([{ "name": name }]);

        let rows: Vec<Entity> = self
            .from("entities")
            .insert(payload)
            .select("id, name")
            .execute_typed()
            .await?;

        rows.get(0)
            .map(|e| e.id)
            .ok_or_else(|| crate::error::SupabasicError::Other("No entity returned".into()))
    }

    /// Fetch all entities
    pub async fn fetch_entities(&self) -> Result<Vec<Entity>> {
        self.from("entities")
            .select("id, name")
            .execute_typed()
            .await
    }
}

use serde::Deserialize;

#[derive(Deserialize)]
pub(crate) struct EntityQuery {
    #[serde(rename = "entityId")]
    pub entity_id: i32,
    #[serde(rename = "entityType")]
    pub entity_type: String,
}
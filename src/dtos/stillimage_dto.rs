use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, NaiveDateTime, Utc};
use tiberius::error::Error;
use utoipa::ToSchema;

use super::misc::{self};

pub trait Dto: Clone + Eq + Send + Sync {
    fn get_identifier(&self) -> i32;
    fn owner_identifier(&self) -> Option<(i32, String)>;
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct StillImageDto {
    pub id: i32,
    pub entity_id: i32,
    pub entity_type: String,
    pub link_type: String,
    pub index: i32,
    pub version: i32,
    pub source_file: String,
    #[schema(value_type = String, format = Date)]
    pub created_time: DateTime<Utc>,
    #[schema(value_type = String, format = Date)]
    pub modified_time: DateTime<Utc>,
    pub locked: bool,
    pub locked_by: i32,
    pub settings: String,
    pub crop_coordindates: String,
    pub crop_mode: i32,
    pub crop_ratio: Option<String>,
    pub tag: Option<String>,
    pub rotation: i32,
    pub flip_horizontal: bool,
    pub masks: Option<String>,
    pub alignment: Option<String>,
    pub status: String,
    pub extended_fields: HashMap<String,Option<String>>
}

impl Dto for StillImageDto{
    fn get_identifier(&self) -> i32 {
        self.id
    }
    
    fn owner_identifier(&self) -> Option<(i32, String)> {
        Some((self.entity_id, self.entity_type.to_string()))
    }
}

impl StillImageDto {
    pub fn from_row(row: &tiberius::Row) -> Result<Self, Error> {
        let mut dto = StillImageDto {
            id: 0,
            entity_id: 0,
            entity_type: String::new(),
            link_type: String::new(),
            index: 0,
            version: 0,
            source_file: String::new(),
            created_time: Utc::now(),
            modified_time: Utc::now(),
            locked: false,
            locked_by: 0,
            settings: String::new(),
            crop_coordindates: String::new(),
            crop_mode: 0,
            crop_ratio: None,
            tag: None,
            rotation: 0,
            flip_horizontal: false,
            masks: None,
            alignment: None,
            status: String::new(),
            extended_fields: HashMap::new(),
        };
        
        for (i, column) in row.columns().iter().enumerate() {
            match column.name() {
                "image_id" => dto.id = row.get(i).unwrap_or_default(),
                "image_entity_id" => dto.entity_id = row.get(i).unwrap_or_default(),
                "image_entity_type" => dto.entity_type = row.get(i).map(|s: &str| s.to_string()).unwrap_or_default(),
                "image_link_type" => dto.link_type = row.get(i).map(|s: &str| s.to_string()).unwrap_or_default(),
                "image_index" => dto.index = row.get(i).unwrap_or_default(),
                "image_version" => dto.version = row.get(i).unwrap_or_default(),
                "image_sourcefile" => dto.source_file = row.get(i).map(|s: &str| s.to_string()).unwrap_or_default(),
                "image_datecreated" =>  dto.created_time = row.get(i).map(|s: NaiveDateTime| DateTime::<Utc>::from_naive_utc_and_offset(s, Utc)).unwrap_or_default(),
                "image_datemodified" =>  dto.modified_time = row.get(i).map(|s| DateTime::<Utc>::from_naive_utc_and_offset(s, Utc)).unwrap_or_default(),
                "image_locked" => dto.locked = row.get(i).unwrap_or_default(),
                "image_lockedby" => dto.locked_by = row.get(i).unwrap_or_default(),
                "image_settings" => dto.settings = row.get(i).map(|s: &str| s.to_string()).unwrap_or_default(),
                "image_cropcoordinates" => dto.crop_coordindates = row.get(i).map(|s: &str| s.to_string()).unwrap_or_default(),
                "image_cropmode" =>  dto.crop_mode = row.get(i).unwrap_or_default(),
                "image_cropratio" => dto.crop_ratio = row.get(i).map(|s: &str| s.to_string()),
                "image_tag" => dto.tag = row.get(i).map(|s: &str| Some(s.to_string())).unwrap_or_default(),
                "image_rotation" => dto.rotation = row.get(i).unwrap_or_default(),
                "image_fliphorizontal" => dto.flip_horizontal = row.get(i).unwrap_or_default(),
                "image_masks" => dto.masks = row.get(i).map(|s: &str| s.to_string()),
                "image_alignment" => dto.alignment = row.get(i).map(|s: &str| s.to_string()),
                "image_status" => dto.status = row.get(i).map(|s: &str| s.to_string()).unwrap_or_default(),
                _ => {
                    let value = misc::sql_to_string(i, column.column_type(), row);
                    dto.extended_fields.insert(column.name().to_string(), value);
                }
            }
        }
        
        Ok(dto)
    }
}

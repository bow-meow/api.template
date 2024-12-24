use std::collections::HashMap;

use anyhow::Error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use super::{misc, stillimage_dto::Dto};


#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Eq, PartialEq, Default)]
#[serde(rename_all = "PascalCase")]
pub struct ArticleDto{
    pub id: i32,
    pub code: Option<String>,
    pub description: Option<String>,
    #[schema(value_type = String, format = Date)]
    pub date_created: DateTime<Utc>,
    #[schema(value_type = String, format = Date)]
    pub date_modified: DateTime<Utc>,
    pub version: Vec<u8>, //timestamp
    pub extended_fields: HashMap<String,Option<String>>
}

impl Dto for ArticleDto{
    fn get_identifier(&self) -> i32 {
        self.id
    }
    
    fn owner_identifier(&self) -> Option<(i32, String)> {
        None
    }
}

impl ArticleDto {
    pub fn from_row(row: &tiberius::Row) -> Result<Self, Error> {
        let mut dto = ArticleDto {
            id: 0,
            code: None,
            description: None,
            date_created: Utc::now(),
            date_modified: Utc::now(),
            version: Vec::new(), //timestamp
            extended_fields: HashMap::new()
        };
        
        for (i, column) in row.columns().iter().enumerate() {
            if !column.name().starts_with("article") {
                continue;
            }
            match column.name() {
                "article_id" => dto.id = row.get(i).unwrap_or_default(),
                "article_code" => dto.code = row.get(i).map(|s: &str| s.to_string()),
                "article_description" => dto.description = row.get(i).map(|s: &str| s.to_string()),
                "article_datecreated" => dto.date_created = row.get(i).map(|s| DateTime::<Utc>::from_naive_utc_and_offset(s, Utc)).unwrap_or_default(),
                "article_datemodified" => dto.date_modified = row.get(i).map(|s| DateTime::<Utc>::from_naive_utc_and_offset(s, Utc)).unwrap_or_default(),
                "article_timestamp" => dto.version = row.get(i).map(|s: &[u8]| s.to_vec()).unwrap_or_default(),
                _ => {
                    let value = misc::sql_to_string(i, column.column_type(), row);
                    dto.extended_fields.insert(column.name().to_string(), value);
                }
            }
        }
        
        Ok(dto)
    }
}
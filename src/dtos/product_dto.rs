use std::collections::HashMap;

use anyhow::Error;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use tiberius::Uuid;
use utoipa::ToSchema;

use super::{article_dto::ArticleDto, misc, stillimage_dto::Dto};


#[derive(Debug, Serialize, Deserialize, ToSchema, Clone, Eq, PartialEq)]
#[serde(rename_all = "PascalCase")]
pub struct ProductDto{
    pub id: i32,
    pub shoot_id: i32,
    pub article: ArticleDto,
    pub code: Option<String>,
    pub description: Option<String>,
    pub internal_code: String,
    pub thumbnail: String,
    pub colour: Option<String>,
    pub status_value: String,
    pub notes: Option<String>,
    pub locked: bool, // is this needed?
    pub locked_by: Option<String>, // is this needed?
    #[schema(value_type = String, format = Date)]
    pub created_time: DateTime<Utc>,
    #[schema(value_type = String, format = Date)]
    pub modified_time: DateTime<Utc>,
    pub selected_spin_id: Option<i32>,
    pub version: Vec<u8>,// timestamp
    pub auth_code: Option<String>,
    pub acc_id: Option<String>,// uniqueid
    pub uid: String,// uniqueid
    pub is_deleted: bool,
    pub spin_count: i32,
    pub image_count: i32,
    pub video_count: i32,
    pub comment_count: i32,
    pub extended_fields: HashMap<String,Option<String>>
}

impl Dto for ProductDto{
    fn get_identifier(&self) -> i32 {
        self.id
    }
    
    fn owner_identifier(&self) -> Option<(i32, String)> {
        None
    }
}

impl ProductDto {
    pub fn from_row(row: &tiberius::Row) -> Result<Self, Error> {
        let mut dto = ProductDto {
            id: 0,
            shoot_id: 0,
            article: ArticleDto::default(),
            code: None,
            description: None,
            internal_code: String::new(),
            thumbnail: String::new(),
            colour: None,
            status_value: String::new(),
            notes: None,
            locked: false,
            locked_by: None,
            created_time: Utc::now(),
            modified_time: Utc::now(),
            selected_spin_id: None,
            version: Vec::new(),
            auth_code: None,
            acc_id: None,
            uid: String::new(),
            is_deleted: false,
            spin_count: 0,
            image_count: 0,
            video_count: 0,
            comment_count: 0,
            extended_fields: HashMap::new()
        };
        
        for (i, column) in row.columns().iter().enumerate() {
            if !column.name().starts_with("prod"){
                continue;
            }
            match column.name() {
                "product_id" => dto.id = row.get(i).unwrap_or_default(),
                "product_shoot_id" => dto.shoot_id = row.get(i).unwrap_or_default(),
                "product_article_id" => dto.article = ArticleDto::from_row(row).unwrap_or_default(),
                "product_code" => dto.code = row.get(i).map(|s: &str| s.to_string()),
                "product_description" => dto.description = row.get(i).map(|s: &str| s.to_string()),
                "product_intcode" => dto.internal_code = row.get(i).map(|s: &str| s.to_string()).unwrap_or_default(),
                "product_thumbnail" => dto.thumbnail = row.get(i).map(|s: &str| s.to_string()).unwrap_or_default(),
                "product_colour" =>  dto.colour = row.get(i).map(|s: &str| s.to_string()),
                "product_status" =>  dto.status_value = row.get(i).map(|s: &str| s.to_string()).unwrap_or_default(),
                "product_notes" => dto.notes = row.get(i).map(|s: &str| s.to_string()),
                "product_locked" => dto.locked = row.get(i).unwrap_or_default(),
                "product_lockedby" => dto.locked_by = row.get(i).map(|s: &str| s.to_string()),
                "product_datecreated" => dto.created_time = row.get(i).map(|s| DateTime::<Utc>::from_naive_utc_and_offset(s, Utc)).unwrap_or_default(),
                "product_datemodified" =>  dto.modified_time = row.get(i).map(|s| DateTime::<Utc>::from_naive_utc_and_offset(s, Utc)).unwrap_or_default(),
                "product_selected_spinme_id" => dto.selected_spin_id = row.get(i),
                "product_timestamp" => dto.version = row.get(i).map(|s: &[u8]| s.to_vec()).unwrap_or_default(),
                "product_authcode" => dto.auth_code = row.get(i).map(|s: &str| s.to_string()),
                "product_acc_id" => dto.acc_id = row.get::<Uuid, _>(i).map(|s| s.to_string()),
                "product_uid" => dto.uid = row.get::<Uuid, _>(i).map(|s| s.to_string()).unwrap_or_default(),
                "product_isdeleted" => dto.is_deleted = row.get(i).unwrap_or_default(),
                "product_spincount" => dto.spin_count = row.get(i).unwrap_or_default(),
                "product_imagecount" => dto.image_count = row.get(i).unwrap_or_default(),
                "product_videocount" => dto.video_count = row.get(i).unwrap_or_default(),
                "product_commentcount" => dto.comment_count = row.get(i).unwrap_or_default(),
                _ => {
                    let value = misc::sql_to_string(i, column.column_type(), row);
                    dto.extended_fields.insert(column.name().to_string(), value);
                }
            }
        }
        
        Ok(dto)
    }
}
use std::sync::Arc;
use tiberius::Query;
use crate::{dtos::StillImageDto, errors::AppError};
use futures::StreamExt;

use super::{caching_repository::{DbKey, Repository}, dbconnection_provider::DbConnectionProvider};

#[derive(Clone)]
pub struct StillImageRepository{
    pub connection_provider: Arc<DbConnectionProvider>
}

impl Repository<DbKey, StillImageDto> for StillImageRepository{
    async fn fetch(&self, key: DbKey) -> Result<Vec<StillImageDto>, AppError> {
        match key{
            DbKey::Id(still_id) =>{
                self.get_by_entity_internal(Some(still_id), None, None).await
            }
            DbKey::EntityIdAndType(entity_id, entity_type) =>{
                self.get_by_entity_internal(None, Some(entity_id), Some(entity_type)).await
            }
        }
    }
    
    async fn insert(&self, dto: StillImageDto) -> Result<StillImageDto, AppError> {
        let mut con = self.connection_provider.get_connection().await?;

        let pararms_count = 17 + dto.extended_fields.len();

        let placeholders: Vec<String> = (1..=pararms_count).map(|i| format!("@P{}", i)).collect();
        let exec_string = format!("exec dbo.imagesext_insert {}", placeholders.join(", "));

        let mut q = Query::new(exec_string);
        q.bind(dto.entity_id);
        q.bind(dto.entity_type.clone());
        q.bind(dto.index);
        q.bind(dto.version);
        q.bind(dto.source_file.clone());
        q.bind(dto.locked);
        q.bind(dto.locked_by);
        q.bind(dto.settings.clone());
        q.bind(dto.crop_coordindates.clone());
        q.bind(dto.crop_mode);
        q.bind(dto.crop_ratio.clone());
        q.bind(dto.tag.clone());
        q.bind(dto.rotation);
        q.bind(dto.flip_horizontal);
        q.bind(dto.masks.clone());
        q.bind(dto.alignment.clone());
        q.bind(dto.status.clone());
        for (_, extendedfield_value) in dto.extended_fields.clone(){
            q.bind(extendedfield_value);
        }

        if let Err(e) = q.execute(&mut con).await{
            return Err(AppError::InternalServerErrorWithMessage(format!("unable to insert stillimage dto. reason: {}", e)));
        }

        Ok(dto)
    }
    
    async fn update(&self, dto: StillImageDto) -> Result<StillImageDto, AppError> {
        let mut con = self.connection_provider.get_connection().await?;


        let pararms_count = 16 + dto.extended_fields.len();

        let placeholders: Vec<String> = (1..=pararms_count).map(|i| format!("@P{}", i)).collect();
        let exec_string = format!("exec dbo.imagesext_update {}", placeholders.join(", "));

        let mut q = Query::new(exec_string);

        q.bind(dto.id);
        q.bind(dto.index);
        q.bind(dto.version);
        q.bind(dto.source_file.clone());
        q.bind(dto.locked);
        q.bind(dto.locked_by);
        q.bind(dto.settings.clone());
        q.bind(dto.crop_coordindates.clone());
        q.bind(dto.crop_mode);
        q.bind(dto.crop_ratio.clone());
        q.bind(dto.tag.clone());
        q.bind(dto.rotation);
        q.bind(dto.flip_horizontal);
        q.bind(dto.masks.clone());
        q.bind(dto.alignment.clone());
        q.bind(dto.status.clone());
        for (_, extendedfield_value) in dto.extended_fields.clone(){
            q.bind(extendedfield_value);
        }
        if let Err(e) = q.execute(&mut con).await{
            return Err(AppError::InternalServerErrorWithMessage(format!("unable to update stillimage. reason: {}", e)));
        }

        Ok(dto)
    }
    
    async fn delete(&self, dto: StillImageDto) -> Result<StillImageDto, AppError> {
        let mut con = self.connection_provider.get_connection().await?;
        let mut q = Query::new("exec dbo.images_delete @P1");
        q.bind(dto.id);
        if let Err(e) = q.execute(&mut con).await{
            return Err(AppError::InternalServerErrorWithMessage(format!("unable to delete stillimage. reason: {}", e)));
        }

        Ok(dto)
    }
}

impl StillImageRepository{
    pub fn new(connection_provider: Arc<DbConnectionProvider>) -> StillImageRepository{
        StillImageRepository{ connection_provider }
    }

    async fn get_by_entity_internal(&self, still_id: Option<i32>, entity_id: Option<i32>, entity_type: Option<String>) -> Result<Vec<StillImageDto>, AppError> {

        let mut con = self.connection_provider.get_connection().await?;
        
        let mut q = Query::new("exec dbo.images_select @P1, @P2, @P3");
        q.bind(still_id);
        q.bind(entity_id);
        q.bind(entity_type);

        let result = q.query( &mut con).await;
        let mut stream = match result {
            Ok(s) => s.into_row_stream(),
            Err(e) => {
                eprintln!("Query execution failed: {}", e);
                return Err(AppError::InternalServerError);
            }
        };

        let mut images = Vec::new();
        while let Some(row_result) = stream.next().await {
            match row_result {
                Ok(row) => {
                    match StillImageDto::from_row(&row) {
                        Ok(dto) => images.push(dto),
                        Err(e) => {
                            return Err(AppError::InternalServerErrorWithMessage(format!("Row conversion failed: {}", e)));
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error while reading row: {}", e);
                    return Err(AppError::InternalServerError);
                }
            }
        }
    
        Ok(images)
    }
    
}
use crate::errors::{AppError, AppResult};
use crate::repositories::caching_repository::{CachingRepository, DbKey, Repository};
use crate::{dtos::StillImageDto, repositories::StillImageRepository};

#[derive(Clone)]
pub struct StillImageService {
    stillimage_repository: CachingRepository<StillImageRepository, StillImageDto>,
}

impl StillImageService{
    pub fn new(stillimage_repository: CachingRepository<StillImageRepository, StillImageDto>) -> Self {
        Self { stillimage_repository }
    }
    pub async fn get_by_id(&self, still_id: i32) -> AppResult<StillImageDto> {
        let images = self.stillimage_repository.fetch(DbKey::Id(still_id)).await?;
        images.into_iter()
        .next()
        .ok_or_else(|| AppError::NotFound("No still images found".to_string()))
    }
    pub async fn get_by_entity(&self, entity_id: i32, entity_type: String) -> AppResult<Vec<StillImageDto>>{
        self.stillimage_repository.fetch(DbKey::EntityIdAndType(entity_id, entity_type)).await
    }

    pub async fn delete_by_id(&self, still_id: i32) -> AppResult<StillImageDto>{
        let dto = self.get_by_id(still_id).await?;
        self.stillimage_repository.delete(dto).await
    }

    pub async fn update(&self, dto: StillImageDto) -> AppResult<StillImageDto>{
        self.stillimage_repository.update(dto).await
    }

    pub async fn insert(&self, dto: StillImageDto) -> AppResult<StillImageDto>{
        self.stillimage_repository.insert(dto).await
    }
}


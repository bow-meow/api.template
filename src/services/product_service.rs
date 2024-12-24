use crate::dtos::ProductDto;
use crate::errors::{AppError, AppResult};
use crate::repositories::caching_repository::{CachingRepository, DbKey, Repository};
use crate::repositories::product_repository::ProductRepository;

// where business logic can occur

#[derive(Clone)]
pub struct ProductService {
    product_repository: CachingRepository<ProductRepository, ProductDto>,
}

impl ProductService{
    pub fn new(product_repository: CachingRepository<ProductRepository, ProductDto>) -> Self {
        Self { product_repository }
    }
    pub async fn get_by_id(&self, product_id: i32) -> AppResult<ProductDto> {
        let products = self.product_repository.fetch(DbKey::Id(product_id)).await?;
        products.into_iter()
        .next()
        .ok_or_else(|| AppError::NotFound("No products found".to_string()))
    }

    pub async fn get_by_entity(&self, entity_id: i32, entity_type: String) -> AppResult<Vec<ProductDto>>{
        self.product_repository.fetch(DbKey::EntityIdAndType(entity_id, entity_type)).await
    }

    pub async fn delete_by_id(&self, still_id: i32) -> AppResult<ProductDto>{
        let dto = self.get_by_id(still_id).await?;
        self.product_repository.delete(dto).await
    }

    pub async fn update(&self, dto: ProductDto) -> AppResult<ProductDto>{
        self.product_repository.update(dto).await
    }

    pub async fn insert(&self, dto: ProductDto) -> AppResult<ProductDto>{
        self.product_repository.insert(dto).await
    }
}


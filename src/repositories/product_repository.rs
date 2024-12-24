use std::sync::Arc;
use tiberius::Query;
use crate::{dtos::ProductDto, errors::AppError};
use futures::StreamExt;

use super::{caching_repository::{DbKey, Repository}, dbconnection_provider::DbConnectionProvider};

#[derive(Clone)]
pub struct ProductRepository{
    pub connection_provider: Arc<DbConnectionProvider>
}

impl Repository<DbKey, ProductDto> for ProductRepository{
    async fn fetch(&self, key: DbKey) -> Result<Vec<ProductDto>, AppError> {
        match key{
            DbKey::Id(product_id) =>{
                self.get_by_entity(Some(product_id), None, None).await
            }
            DbKey::EntityIdAndType(entity_id, entity_type) =>{
                match entity_type.as_str(){
                    "shoot" => self.get_by_entity(None, Some(entity_id), None).await,
                    "article" => self.get_by_entity(None, None, Some(entity_id)).await,
                    _ => Err(AppError::InternalServerErrorWithMessage(format!("the entitytype {} is not handled by the product repository", entity_type)))
                }
            }
        }
    }
    
    async fn insert(&self, dto: ProductDto) -> Result<ProductDto, AppError> {
        let mut con = self.connection_provider.get_connection().await?;

        let pararms_count = 14 + dto.extended_fields.len();

        let placeholders: Vec<String> = (1..=pararms_count).map(|i| format!("@P{}", i)).collect();
        let exec_string = format!("exec dbo.productsext_insert {}", placeholders.join(", "));

        let mut q = Query::new(exec_string);
        q.bind(dto.shoot_id);
        q.bind(dto.code.clone());
        q.bind(dto.description.clone());
        q.bind(dto.internal_code.clone());
        q.bind(dto.thumbnail.clone());
        q.bind(dto.colour.clone());
        q.bind(dto.status_value.clone());
        q.bind(dto.notes.clone());
        q.bind(dto.locked);
        q.bind(dto.locked_by.clone());
        q.bind(dto.selected_spin_id);
        q.bind(dto.auth_code.clone());
        q.bind(dto.acc_id.clone());
        q.bind(dto.uid.clone());
        for (_, extendedfield_value) in dto.extended_fields.clone(){
            q.bind(extendedfield_value);
        }

        if let Err(e) = q.execute(&mut con).await{
            return Err(AppError::InternalServerErrorWithMessage(format!("unable to insert product. reason: {}", e)));
        }

        Ok(dto)
    }
    
    async fn update(&self, dto: ProductDto) -> Result<ProductDto, AppError> {
        let mut con = self.connection_provider.get_connection().await?;

        let pararms_count = 16 + dto.extended_fields.len();

        let placeholders: Vec<String> = (1..=pararms_count).map(|i| format!("@P{}", i)).collect();
        let exec_string = format!("exec dbo.imagesext_update {}", placeholders.join(", "));

        let mut q = Query::new(exec_string);

        q.bind(dto.id);
        q.bind(dto.article.id);
        q.bind(dto.code.clone());
        q.bind(dto.description.clone());
        q.bind(dto.internal_code.clone());
        q.bind(dto.thumbnail.clone());
        q.bind(dto.colour.clone());
        q.bind(dto.status_value.clone());
        q.bind(dto.notes.clone());
        q.bind(dto.locked);
        q.bind(dto.locked_by.clone());
        q.bind(dto.selected_spin_id);
        q.bind(dto.auth_code.clone());
        q.bind(dto.acc_id.clone());
        q.bind(dto.uid.clone());
        q.bind(false); // set status
        for (_, extendedfield_value) in dto.extended_fields.clone(){
            q.bind(extendedfield_value);
        }
        if let Err(e) = q.execute(&mut con).await{
            return Err(AppError::InternalServerErrorWithMessage(format!("unable to update product. reason: {}", e)));
        }

        Ok(dto)
    }
    
    async fn delete(&self, dto: ProductDto) -> Result<ProductDto, AppError> {
        let mut con = self.connection_provider.get_connection().await?;
        let mut q = Query::new("exec dbo.images_delete @P1, @P2");
        q.bind(dto.id);
        q.bind(true); // soft delete
        if let Err(e) = q.execute(&mut con).await{
            return Err(AppError::InternalServerErrorWithMessage(format!("unable to delete product. reason: {}", e)));
        }

        Ok(dto)
    }
}

impl ProductRepository{
    pub fn new(connection_provider: Arc<DbConnectionProvider>) -> ProductRepository{
        ProductRepository{ connection_provider }
    }

    async fn get_by_entity(&self, product_id: Option<i32>, shoot_id: Option<i32>, article_id: Option<i32>) -> Result<Vec<ProductDto>, AppError> {

        let mut con = self.connection_provider.get_connection().await?;
        
        let mut q = Query::new("exec dbo.products_select @P1, @P2, @P3, @P4");
        q.bind(product_id);
        q.bind(shoot_id);
        q.bind(article_id);
        q.bind(false); // whether to get deleted products

        let result = q.query( &mut con).await;
        let mut stream = match result {
            Ok(s) => s.into_row_stream(),
            Err(e) => {
                eprintln!("Query execution failed: {}", e);
                return Err(AppError::InternalServerError);
            }
        };

        let mut products = Vec::new();
        while let Some(row_result) = stream.next().await {
            match row_result {
                Ok(row) => {
                    match ProductDto::from_row(&row) {
                        Ok(dto) => products.push(dto),
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
    
        Ok(products)
    }
    
}
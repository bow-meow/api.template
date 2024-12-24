use std::sync::Arc;
use deadpool_tiberius::Pool;
use crate::repositories::{caching_repository::CachingRepository, dbconnection_provider::DbConnectionProvider, product_repository::ProductRepository, StillImageRepository};
use super::{product_service::ProductService, StillImageService};

#[derive(Clone)]
pub struct ServiceRegister {
    // either register all repos here or inject them one by one to the routes. this way is easier
    pub stillimage_service: Option<StillImageService>,
    pub product_service: Option<ProductService>
}
impl ServiceRegister {
    pub fn new(pool: Pool) -> Self {
        let connection_provider = Arc::new(DbConnectionProvider::new(pool));

        let stillimage_repository = StillImageRepository::new(Arc::clone(&connection_provider));
        let caching_still_repo = CachingRepository::new(Arc::new(stillimage_repository), 10_000);
        let stillimage_service = StillImageService::new(caching_still_repo);

        let product_repository = ProductRepository::new(Arc::clone(&connection_provider));
        let caching_product_repo = CachingRepository::new(Arc::new(product_repository), 10_000);
        let product_service = ProductService::new(caching_product_repo);

        Self {
            stillimage_service: Some(stillimage_service),
            product_service: Some(product_service)
        }
    }
}
use std::sync::Arc;
use moka::future::Cache;
use crate::{dtos::stillimage_dto::Dto, errors::AppError};

#[derive(Clone, Eq, PartialEq, Hash)]
pub enum DbKey{
    Id(i32),
    EntityIdAndType(i32, String),
}

pub trait Repository<DbKey, V> {
    async fn fetch(&self, key: DbKey) -> Result<Vec<V>, AppError>;
    async fn insert(&self, dto: V) -> Result<V, AppError>;
    async fn update(&self, dto: V) -> Result<V, AppError>;
    async fn delete(&self, dto: V) -> Result<V, AppError>;
}

#[derive(Clone)]
pub struct CachingRepository<R, V>
where 
    R: Repository<DbKey, V> + Send + Sync,
    V: Dto
{
    cache_max_capacity: u64,
    inner: Arc<R>,
    cache: Cache<DbKey, V>,
    owner_lookup: Cache<DbKey, Vec<i32>>
}

impl<R, V> CachingRepository<R, V>
where
    R: Repository<DbKey,V> + Send + Sync,
    V: Dto + 'static
{
    pub fn new(inner: Arc<R>, cache_size: u64) -> Self {
        let cache = Cache::<DbKey, V>::builder()
        .max_capacity(cache_size)
        .build();
        Self {
            cache_max_capacity: cache_size,
            owner_lookup: Cache::new(cache_size),
            inner,
            cache,
        }
    }
}

impl<R, V> Repository<DbKey, V> for CachingRepository<R, V>
where
    R: Repository<DbKey, V> + Send + Sync,
    V: Dto + 'static
{
    async fn fetch(&self, key: DbKey) -> Result<Vec<V>, AppError> {
        // this might need a lock if we want to keep a mega cache
        match key{
            DbKey::Id(_) => {
                if let Some(cached_value) = self.cache.get(&key).await {
                    return Ok(vec!(cached_value.clone()));
                }
                let values = self.inner.fetch(key.clone()).await?;
                let first = values.into_iter()
                .next()
                .ok_or_else(|| AppError::NotFound("stillimage not found".to_string()));

                match first{
                    Ok(value) => {
                        self.cache.insert(key, value.clone()).await;
                        Ok(vec!(value))
                    }
                    Err(err) => Err(err)
                }
            },
            DbKey::EntityIdAndType(_, _ ) =>{
                if let Some(ids) = self.owner_lookup.get(&key).await{
                    if ids.len() as u64 <= self.cache_max_capacity{
                        let mut cached_items = Vec::new();
                        for id in ids.iter() {
                            let db_key = DbKey::Id(*id);
                            if let Some(cached_value) = self.cache.get(&db_key).await {
                                cached_items.push(cached_value.clone());
                            }
                        }
                        if !cached_items.is_empty() {
                            return Ok(cached_items);
                        }
                    }

                }
                let items = self.inner.fetch(key.clone()).await?;

                for item in &items {
                    let item_key = DbKey::Id(item.get_identifier());
                    self.cache.insert(item_key, item.clone()).await;
                }
                self.owner_lookup.insert(key, items.iter().map(|item| item.get_identifier()).collect()).await;
        
                Ok(items)
            }
        }
    }
    
    async fn insert(&self, dto: V) -> Result<V, AppError> {
        let dto = self.inner.insert(dto).await?;
    
        self.cache.insert(DbKey::Id(dto.get_identifier()), dto.clone()).await;
    
        if let Some((entity_id, entity_type)) = dto.owner_identifier() {
            let key = DbKey::EntityIdAndType(entity_id, entity_type);
            match self.owner_lookup.get(&key).await {
                Some(mut ids) => {
                    ids.push(dto.get_identifier());
                    self.owner_lookup.insert(key, ids).await;
                }
                None => {
                    self.owner_lookup.insert(key, vec![dto.get_identifier()]).await;
                }
            }
        }
        Ok(dto)
    }
    
    async fn update(&self, dto: V) -> Result<V, AppError> {
        let dto = self.inner.update(dto).await?;
        
        self.cache.insert(DbKey::Id(dto.get_identifier()), dto.clone()).await;

        if let Some((entity_id, entity_type)) = dto.owner_identifier() {
            let key = DbKey::EntityIdAndType(entity_id, entity_type);
            match self.owner_lookup.get(&key).await {
                Some(mut ids) => {
                    ids.push(dto.get_identifier());
                    self.owner_lookup.insert(key, ids).await;
                }
                None => {
                    self.owner_lookup.insert(key, vec![dto.get_identifier()]).await;
                }
            }
        }
        Ok(dto)
    }
    
    async fn delete(&self, dto: V) -> Result<V, AppError> {
        let dto = self.inner.delete(dto).await?;
        self.cache.remove(&DbKey::Id(dto.get_identifier())).await;

        if let Some((entity_id, entity_type)) = dto.owner_identifier() {
            let key = DbKey::EntityIdAndType(entity_id, entity_type);
            if let Some(mut ids)  = self.owner_lookup.get(&key).await {
                ids.retain(|x| *x != dto.get_identifier());
                self.owner_lookup.insert(key, ids).await;
            }
        }
        Ok(dto)
    }
}
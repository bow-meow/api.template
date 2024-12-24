use deadpool_tiberius::{deadpool::managed::Object, Manager, Pool};
use crate::errors::AppError;

#[derive(Clone)]
pub struct DbConnectionProvider {
    pool: Pool,
}

impl DbConnectionProvider {
    pub fn new(pool: Pool) -> Self {
        DbConnectionProvider { pool }
    }

    pub async fn get_connection(&self) -> Result<Object<Manager>, AppError> {
        let con = match self.pool.get().await {
            Ok(conn) => conn,
            Err(e) => {
                eprintln!("Failed to get connection: {}", e);
                return Err(AppError::InternalServerError);
            }
        };
        Ok(con)
    }
}

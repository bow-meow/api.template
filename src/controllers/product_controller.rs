use axum::{extract::{self, Path, Query, State}, http, response::IntoResponse, routing::{delete, get, post, put}, Json, Router};
use reqwest::StatusCode;

use crate::{dtos::ProductDto, errors::{AppError, AppResult}, services::product_service::ProductService};

use super::misc::EntityQuery;

pub fn router() -> Router<ProductService> {
    Router::new()
    .route("/products/:id", get(get_by_id))
    .route("/products/entity", get(get_by_entity))
    .route("/products/:id", delete(delete_by_id))
    .route("/products", put(update))
    .route("/products", post(insert))
}

/// Gets a product by id
#[utoipa::path(
    get,
    path = "/products/:id",
    responses(
        (status = 200, description = "Successfully retrieved product", body = [ProductDto]),
        (status = 500, description = "Internal Server Error"),
    ),
    params(
        ("productId" = i32, Path, description = "the product id"),
    ),
    tag = "products",
)]
pub async fn get_by_id(
    Path(product_id): Path<i32>,
    State(product_service): State<ProductService>,
) -> AppResult<Json<ProductDto>> {
    let product = product_service.get_by_id(product_id).await?;

    Ok(Json(product))
}

/// Gets a list of products by entity_id and entity_type
/// entity_type can be 'shoot' or 'article'
#[utoipa::path(
    get,
    path = "/products/entity",
    responses(
        (status = 200, description = "Successfully retrieved product", body = [ProductDto]),
        (status = 500, description = "Internal Server Error"),
    ),
    params(
        ("id" = i32, Path, description = "the product id"),
    ),
    tag = "products",
)]
pub async fn get_by_entity(
    Query(params): Query<EntityQuery>,
    State(product_service): State<ProductService>,
) -> AppResult<Json<Vec<ProductDto>>> {
    let products = product_service.get_by_entity(params.entity_id, params.entity_type).await?;

    Ok(Json(products))
}

/// deletes a product by id
#[utoipa::path(
    delete,
    path = "/products/:id",
    responses(
        (status = 200, description = "Successfully deleted product", body = [ProductDto]), // wrong
        (status = 500, description = "Internal Server Error"),
    ),
    params(
        ("id" = i32, Path, description = "the product id"),
    ),
    tag = "products",
)]
pub async fn delete_by_id(
    Path(product_id): Path<i32>,
    State(product_service): State<ProductService>
) -> Result<impl IntoResponse, AppError> {
    product_service.delete_by_id(product_id).await?;
    Ok(http::status::StatusCode::OK)
}

/// updates a product in the database
#[utoipa::path(
    put,
    path = "/products",
    responses(
        (status = 200, description = "Successfully deleted product", body = [ProductDto]), // wrong
        (status = 500, description = "Internal Server Error"),
    ),
    params(
        ("id" = i32, Path, description = "the product id"),
    ),
    tag = "products",
)]
pub async fn update(
    State(product_service): State<ProductService>,
    extract::Json(dto): extract::Json<ProductDto>,
) -> Result<StatusCode, AppError> {
    product_service.update(dto).await?;

    Ok(http::status::StatusCode::OK)
}

/// inserts a product to the database
#[utoipa::path(
    delete,
    path = "/products",
    responses(
        (status = 200, description = "Successfully deleted product", body = [ProductDto]),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "products",
)]
pub async fn insert(
    State(product_service): State<ProductService>,
    axum::extract::Json(dto): Json<ProductDto>,
) -> Result<impl IntoResponse, AppError> {
    product_service.insert(dto).await?;

    Ok(http::status::StatusCode::OK)
}
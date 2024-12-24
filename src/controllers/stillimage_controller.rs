use axum::{extract::{self, Path, Query, State}, http, response::IntoResponse, routing::{delete, get, post, put}, Json, Router};
use reqwest::StatusCode;

use crate::{dtos::StillImageDto, errors::{AppError, AppResult}, services::StillImageService};

use super::misc::EntityQuery;

pub fn router() -> Router<StillImageService> {
    Router::new()
    .route("/stills/:id", get(get_by_id))
    .route("/stills/entity", get(get_by_entity))
    .route("/stills/:id", delete(delete_by_id))
    .route("/stills", put(update))
    .route("/stills", post(insert))
}

/// Gets a stillimage by id
#[utoipa::path(
    get,
    path = "/stills/:id",
    responses(
        (status = 200, description = "Successfully retrieved stillimage", body = [StillImageDto]),
        (status = 500, description = "Internal Server Error"),
    ),
    params(
        ("entityId" = i32, Path, description = "The ID of the entity the stillimage belongs to."),
        ("entityType" = String, Path, description = "Entity type can be: product, productgroup, or customview."),
    ),
    tag = "stills",
)]
pub async fn get_by_id(
    Path(still_id): Path<i32>,
    State(stillimage_service): State<StillImageService>,
) -> AppResult<Json<StillImageDto>> {
    let stillimage = stillimage_service.get_by_id(still_id).await?;

    Ok(Json(stillimage))
}

/// Gets a list of stills by entity_id and entity_type
/// entity_type can be 'product' or 'productgroup' or 'customview'
#[utoipa::path(
    get,
    path = "/stills/entity",
    responses(
        (status = 200, description = "Successfully retrieved stillimage", body = [StillImageDto]),
        (status = 500, description = "Internal Server Error"),
    ),
    params(
        ("id" = i32, Path, description = "the image id"),
    ),
    tag = "stills",
)]
pub async fn get_by_entity(
    Query(params): Query<EntityQuery>,
    State(stillimage_service): State<StillImageService>,
) -> AppResult<Json<Vec<StillImageDto>>> {
    let stillimage = stillimage_service.get_by_entity(params.entity_id, params.entity_type).await?;

    Ok(Json(stillimage))
}

/// deletes a stillimage by id
#[utoipa::path(
    delete,
    path = "/stills/:id",
    responses(
        (status = 200, description = "Successfully deleted stillimage", body = [StillImageDto]), // wrong
        (status = 500, description = "Internal Server Error"),
    ),
    params(
        ("id" = i32, Path, description = "the image id"),
    ),
    tag = "stills",
)]
pub async fn delete_by_id(
    Path(still_id): Path<i32>,
    State(stillimage_service): State<StillImageService>
) -> Result<impl IntoResponse, AppError> {
    stillimage_service.delete_by_id(still_id).await?;
    Ok(http::status::StatusCode::OK)
}

/// updates a stillimage in the database
#[utoipa::path(
    put,
    path = "/stills",
    responses(
        (status = 200, description = "Successfully deleted stillimage", body = [StillImageDto]), // wrong
        (status = 500, description = "Internal Server Error"),
    ),
    params(
        ("id" = i32, Path, description = "the image id"),
    ),
    tag = "stills",
)]
pub async fn update(
    State(stillimage_service): State<StillImageService>,
    extract::Json(dto): extract::Json<StillImageDto>,
) -> Result<StatusCode, AppError> {
    stillimage_service.update(dto).await?;

    Ok(http::status::StatusCode::OK)
}

/// inserts a stillimage to the database
#[utoipa::path(
    delete,
    path = "/stills",
    responses(
        (status = 200, description = "Successfully deleted stillimage", body = [StillImageDto]),
        (status = 500, description = "Internal Server Error"),
    ),
    tag = "stills",
)]
pub async fn insert(
    State(stillimage_service): State<StillImageService>,
    axum::extract::Json(dto): Json<StillImageDto>,
) -> Result<impl IntoResponse, AppError> {
    stillimage_service.insert(dto).await?;

    Ok(http::status::StatusCode::OK)
}
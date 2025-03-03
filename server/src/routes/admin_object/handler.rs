use crate::entity::object::ObjectsPaginated;
use crate::entity::pagination::Pagination;
use crate::entity::user::User;
use crate::error::api_error::ApiError;
use crate::state::object_state::ObjectState;
use axum::Extension;
use axum::{extract::State, Json};
use axum_extra::extract::OptionalQuery;
use validator::Validate;

pub async fn admin_get_object_list(
    State(state): State<ObjectState>,
    OptionalQuery(pagination): OptionalQuery<Pagination>,
    Extension(_): Extension<User>,
) -> Result<Json<ObjectsPaginated>, ApiError> {
    let pagination = pagination.unwrap_or_default();
    pagination.validate().unwrap();

    let res: ObjectsPaginated = state.object_service.admin_get_object_list(pagination).await?;
    Ok(Json(res))
}

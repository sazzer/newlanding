use crate::http::{hal::HalRespondable, problem::Problem, problem::NOT_FOUND, Response};
use crate::users::{GetUserUseCase, UserId};
use actix_web::web::{Data, Path};
use std::sync::Arc;

/// Get the requested user and return it to the client
///
/// # Parameters
/// - `path` - The parsed URL path, containing the requested user ID
/// - `get_user_use_case` - The use case to use for getting user records
///
/// # Returns
/// The HTTP Response. Either the user as a HAL document or else a Problem indicting why the user couldn't be loaded.
pub async fn handle(
    path: Path<String>,
    get_user_use_case: Data<Arc<GetUserUseCase>>,
) -> Result<Response<HalRespondable>, Problem> {
    let user_id = path.0.parse::<UserId>().map_err(|e| {
        tracing::warn!(e = ?e, "Failed to parse User ID");
        Problem::from(NOT_FOUND)
    })?;

    let user = get_user_use_case
        .get_user_by_id(user_id)
        .await
        .ok_or_else(|| Problem::from(NOT_FOUND))?;

    Ok(user.into())
}

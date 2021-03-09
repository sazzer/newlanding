use crate::http::{hal::HalRespondable, problem::Problem, problem::NOT_FOUND, Response};
use crate::users::{GetUserUseCase, UserId};
use actix_web::web::{Data, Path};
use std::sync::Arc;

pub async fn handle(
    path: Path<String>,
    get_user_use_case: Data<Arc<GetUserUseCase>>,
) -> Result<Response<HalRespondable>, Problem> {
    let user_id = path.0.parse::<UserId>().map_err(|e| {
        tracing::warn!(e = ?e, "Failed to parse User ID");
        Problem::from(NOT_FOUND)
    })?;

    let user = get_user_use_case
        .get_user_by_id(&user_id)
        .await
        .ok_or_else(|| Problem::from(NOT_FOUND))?;

    Ok(user.into())
}

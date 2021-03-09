use super::model::UserModel;
use crate::http::{
    hal::HalDocument, problem::Problem, problem::NOT_FOUND, Response, SimpleRespondable,
};
use crate::users::{GetUserUseCase, UserId};
use actix_http::http::{
    header::{CacheControl, CacheDirective},
    StatusCode,
};
use actix_web::web::{Data, Path};
use std::sync::Arc;

pub async fn handle(
    path: Path<String>,
    get_user_use_case: Data<Arc<GetUserUseCase>>,
) -> Result<Response<SimpleRespondable<HalDocument>>, Problem> {
    let user_id = path.0.parse::<UserId>().unwrap();
    let user = get_user_use_case.get_user_by_id(&user_id).await;

    let hal_document = HalDocument::new(UserModel {});

    // SimpleRespondable::from(hal_document)
    //     .with_status_code(StatusCode::OK)
    //     .with_header(CacheControl(vec![
    //         CacheDirective::Public,
    //         CacheDirective::MaxAge(3600),
    //     ]))
    //     .into();

    Err(NOT_FOUND.into())
}

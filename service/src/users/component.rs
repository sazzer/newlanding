use std::sync::Arc;

use crate::server::RouteConfigurer;
use actix_web::web::ServiceConfig;

use super::{repository::UserRepository, GetUserUseCase};

pub struct Component {
    get_user_use_case: Arc<GetUserUseCase>,
}

pub fn new() -> Arc<Component> {
    let repository = UserRepository::new();

    let component = Component {
        get_user_use_case: Arc::new(GetUserUseCase::new(repository)),
    };

    Arc::new(component)
}

impl RouteConfigurer for Component {
    fn configure_routes(&self, config: &mut ServiceConfig) {
        config.data(self.get_user_use_case.clone());
        super::http::configure_routes(config);
    }
}

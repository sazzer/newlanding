use super::{
    auth0::{ClientId, ClientSecret, Domain, UserRepository},
    GetUserUseCase,
};
use crate::server::RouteConfigurer;
use actix_web::web::ServiceConfig;
use std::sync::Arc;

/// Users component for working with users, working in terms of Auth0.
pub struct Component {
    get_user_use_case: Arc<GetUserUseCase>,
}

/// Create a new instance of the Users component
///
/// # Parameters
/// - `domain` - The Auth0 domain to work with
/// - `client_id` - The Auth0 Client ID
/// - `client_secret` - The Auth0 Client Secret
///
/// # Returns
/// The Users component
pub fn new<D, I, S>(domain: D, client_id: I, client_secret: S) -> Arc<Component>
where
    D: Into<String>,
    I: Into<String>,
    S: Into<String>,
{
    let repository = UserRepository::new(
        Domain::new(domain),
        ClientId::new(client_id),
        ClientSecret::new(client_secret),
    );

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

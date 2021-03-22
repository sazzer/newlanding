use crate::server::RouteConfigurer;
use actix_web::web::ServiceConfig;
use std::sync::Arc;

use super::{auth0::AccessTokenParser, auth0::Domain};

/// Users component for authorization, working in terms of Auth0.
pub struct Component {
    access_token_parser: Arc<AccessTokenParser>,
}

/// Create a new instance of the Authorization component
///
/// # Parameters
/// - `domain` - The Auth0 domain to work with
/// - `audience` - The API Audience
///
/// # Returns
/// The Authorization component
pub fn new<D, A>(domain: D, audience: A) -> Arc<Component>
where
    D: Into<String>,
    A: Into<String>,
{
    let access_token_parser = Arc::new(AccessTokenParser::new(
        Domain::new(domain.into()),
        audience.into(),
    ));

    let component = Component {
        access_token_parser,
    };

    Arc::new(component)
}

impl RouteConfigurer for Component {
    fn configure_routes(&self, config: &mut ServiceConfig) {
        config.data(self.access_token_parser.clone());
    }
}

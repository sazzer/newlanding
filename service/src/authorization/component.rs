use crate::server::RouteConfigurer;
use actix_web::web::ServiceConfig;
use std::sync::Arc;

/// Users component for authorization, working in terms of Auth0.
pub struct Component {}

/// Create a new instance of the Authorization component
///
/// # Parameters
/// - `domain` - The Auth0 domain to work with
/// - `audience` - The API Audience
///
/// # Returns
/// The Authorization component
pub fn new<D, A>(_domain: D, _audience: A) -> Arc<Component>
where
    D: Into<String>,
    A: Into<String>,
{
    let component = Component {};

    Arc::new(component)
}

impl RouteConfigurer for Component {
    fn configure_routes(&self, _config: &mut ServiceConfig) {}
}

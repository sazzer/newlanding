mod access_token;
mod domain;
mod get_user;

pub use access_token::{ClientId, ClientSecret};
pub use domain::Domain;

/// Repository of user details as available in Auth0.
pub struct UserRepository {
    /// The means to retrieve an access token for working with Auth0.
    access_token_retriever: access_token::Retriever,
}

impl UserRepository {
    /// Create a new Auth0 User Repository
    ///
    /// # Parameters
    /// - `domain` - The Auth0 domain to work with
    /// - `client_id` - The Auth0 Client ID
    /// - `client_secret` - The Auth0 Client Secret
    ///
    /// # Returns
    /// The repository
    pub fn new(domain: Domain, client_id: ClientId, client_secret: ClientSecret) -> Self {
        let access_token_retriever = access_token::Retriever::new(domain, client_id, client_secret);

        Self {
            access_token_retriever,
        }
    }
}

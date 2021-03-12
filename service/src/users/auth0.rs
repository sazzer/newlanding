mod access_token;
mod domain;
mod get_user;

pub use access_token::{ClientId, ClientSecret};
pub use domain::Domain;
use reqwest::Client;

/// Repository of user details as available in Auth0.
pub struct UserRepository {
    /// The means to retrieve an access token for working with Auth0.
    access_token_retriever: access_token::Retriever,
    /// The Auth0 domain.
    domain: Domain,
    /// The HTTP client to use to talk to Auth0.
    client: Client,
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
        let access_token_retriever =
            access_token::Retriever::new(domain.clone(), client_id, client_secret);
        let client = Client::new();

        Self {
            access_token_retriever,
            domain,
            client,
        }
    }
}

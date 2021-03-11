mod access_token;
mod domain;
mod get_user;

pub use access_token::{ClientId, ClientSecret};
pub use domain::Domain;

pub struct UserRepository {
    access_token_retriever: access_token::Retriever,
}

impl UserRepository {
    pub fn new(domain: Domain, client_id: ClientId, client_secret: ClientSecret) -> Self {
        let access_token_retriever = access_token::Retriever::new(domain, client_id, client_secret);

        Self {
            access_token_retriever,
        }
    }
}

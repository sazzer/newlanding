/// Representation of the Auth0 domain name.
pub struct Domain(String);

impl Domain {
    /// Create a new instance of the `Domain` struct
    ///
    /// # Parameters
    /// - `domain` - The actual domain, as a string
    pub fn new<S>(domain: S) -> Self
    where
        S: Into<String>,
    {
        Self(domain.into())
    }

    /// Helper to build a full URL for this domain
    ///
    /// # Parameters
    /// - `url` - The URL to use under this domain
    ///
    /// # Returns
    /// The full url
    pub fn build_url<S>(&self, url: S) -> String
    where
        S: Into<String>,
    {
        format!("{}{}", self.0, url.into())
    }
}

#[cfg(test)]
mod tests {
    use super::Domain;
    use assert2::check;

    #[test]
    fn build_url() {
        let domain = Domain::new("http://example.eu.auth0.com");
        let url = domain.build_url("/api/v2/");

        check!(url == "http://example.eu.auth0.com/api/v2/");
    }
}

use uritemplate::UriTemplate;

/// Representation of the Auth0 domain name.
#[derive(Debug, Clone)]
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

    /// Helper to build a URL Template for this domain
    ///
    /// # Parameters
    /// - `url` - The URL template to use under this domain
    ///
    /// # Returns
    /// The URL Template
    pub fn build_url_template<S>(&self, url: S) -> UriTemplate
    where
        S: Into<String>,
    {
        UriTemplate::new(&self.build_url(url))
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

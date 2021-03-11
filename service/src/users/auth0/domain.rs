pub struct Domain(String);

impl Domain {
    pub fn new<S>(domain: S) -> Self
    where
        S: Into<String>,
    {
        Self(domain.into())
    }

    pub fn build_url<S>(&self, url: S) -> String
    where
        S: Into<String>,
    {
        format!("{}{}", self.0, url.into())
    }
}

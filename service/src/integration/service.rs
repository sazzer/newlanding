use actix_http::Request;

use crate::service::{testing::TestResponse, Service};

pub struct TestService {
    service: Service,
}

impl TestService {
    pub async fn new() -> Self {
        let _ = env_logger::try_init();

        let cfg = crate::settings::Settings {
            port: 0,
            auth0_domain: mockito::server_url(),
            auth0_client_id: "testAuth0ClientId".to_owned(),
            auth0_client_secret: "testAuth0ClientSecret".to_owned(),
        };

        let service = Service::new(cfg).await;
        Self { service }
    }

    pub async fn inject(&self, req: Request) -> TestResponse {
        self.service.inject(req).await
    }
}

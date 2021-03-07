pub mod component;
mod span;

use actix_cors::Cors;
use actix_http::http::header;
use actix_web::{middleware::Logger, App, HttpServer};

pub struct Server {
    port: u16,
}

impl Server {
    pub async fn start(&self) {
        let address = format!("0.0.0.0:{}", self.port);

        tracing::debug!(address = ?address, "Starting HTTP server");

        HttpServer::new(move || {
            let app = App::new()
                .wrap(Logger::default())
                .wrap(
                    Cors::default()
                        .allow_any_origin()
                        .allow_any_method()
                        .allow_any_header()
                        .expose_headers(vec![header::ETAG, header::LOCATION, header::LINK]),
                )
                .wrap(span::Span);

            tracing::trace!("Built listener");

            app
        })
        .bind(address)
        .unwrap()
        .run()
        .await
        .unwrap();
    }
}

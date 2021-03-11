#![deny(clippy::all, clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]

mod home;
mod http;
#[cfg(test)]
mod integration;
mod model;
mod server;
mod service;
mod settings;
mod users;

pub use service::Service;
pub use settings::Settings;

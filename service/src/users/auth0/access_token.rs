mod retriever;

use serde::Deserialize;
#[derive(Debug, Deserialize, PartialEq)]
pub struct AccessToken(String);

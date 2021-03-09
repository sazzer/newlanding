use super::HalDocument;
use crate::http::SimpleRespondable;

pub type HalRespondable = SimpleRespondable<HalDocument>;

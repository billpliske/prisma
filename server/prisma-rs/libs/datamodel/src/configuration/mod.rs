mod source;
pub use source::*;
mod generator;
pub use generator::*;
mod json;
pub use json::*;

use serde::{Deserialize, Serialize};

pub struct Configuration {
    pub generators: Vec<Generator>,
    pub datasources: Vec<Box<Source>>,
}

impl Configuration {
    pub fn to_serializeable(&self) -> SerializeableMcf {
        SerializeableMcf {
            generators: generator::generators_to_json_value(&self.generators),
            datasources: source::render_sources_to_json_value(&self.datasources),
        }
    }
}

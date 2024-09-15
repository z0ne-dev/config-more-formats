use crate::util::{extract_root_table, from_value, Val};
use config::{Format, Map, Value};
use std::error::Error;

pub struct Properties;

impl Format for Properties {
    fn parse(&self, uri: Option<&String>, text: &str) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
        let properties: Val = serde_java_properties::from_str(text)
            .map_err(|err| Box::new(err) as Box<dyn Error + Send + Sync>)?;

        extract_root_table(from_value(uri, properties))
    }
}

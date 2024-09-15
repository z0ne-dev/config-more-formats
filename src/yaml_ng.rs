use crate::util::{extract_root_table, from_value, Val};
use config::{Format, Map, Value};
use std::error::Error;

pub struct YamlNg;

impl Format for YamlNg {
    fn parse(&self, uri: Option<&String>, text: &str) -> Result<Map<String, Value>, Box<dyn Error + Send + Sync>> {
        let hjson: Val = serde_yaml_ng::from_str(text)
            .map_err(|err| Box::new(err) as Box<dyn Error + Send + Sync>)?;

        extract_root_table(from_value(uri, hjson))
    }
}